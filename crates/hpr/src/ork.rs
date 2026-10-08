//! Flying what an OpenRocket `.ork` file says about staging and recovery.
//!
//! `hpr_io` reads each of a `.ork` file's powered separations as an [`hpr_io::ork::Staging`],
//! naming the motor it is timed from by its mount, since `hpr_io` does not depend on `hpr_sim`.
//! [`separation`] turns one into the flight's [`hpr_sim::Separation`], naming that motor by its
//! index among the assembled motors, and [`separations`] a configuration's list. The flight also
//! needs a recovery device on each of the bodies the separations make (the decision record on
//! staging, [ADR-074][adr-074]):
//! [`separated_recovery`] puts the file's devices on their parts and adds the tumbles, as
//! `hpr sim` flies it; the example `ork_two_stage` in this crate shows the pieces. The decision record for reading a `.ork` file's
//! staging is [ADR-076][adr-076].
//!
//! [adr-074]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-074-ignition-times-and-powered-staging-the-sustainer-flies-on-as-a-rigid-body-2026-09-25
//! [adr-076]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-076-a-ork-files-ignitions-and-one-powered-separation-flown-against-openrocket-2026-09-25

use hpr_design::{Assembly, DesignError, Part};
use hpr_io::ork::{DeployEvent, DeviceKind, Dimension, Recovery, Staging, StagingTrigger};
use hpr_motor::Delay;
use hpr_sim::recovery::{MIN_STREAMER_ASPECT_RATIO, StreamerModel};
use hpr_sim::{Device, DeviceDrag, Separation, SimError, Trigger};

/// The flight's separation for `staging`, with the motors of `assembly`, the configuration the
/// `.ork` file's staging belongs to, assembled.
///
/// # Errors
///
/// [`SimError::Domain`] if the mount a separation is timed from places no lit motor in `assembly`
/// (it is not that configuration's, or every tube of it is set to fail), or if `staging` holds a trigger this function does not know.
pub fn separation(staging: &Staging, assembly: &Assembly) -> Result<Separation, SimError> {
    let trigger = match &staging.trigger {
        StagingTrigger::Time { time_s } => Trigger::Time { time_s: *time_s },
        StagingTrigger::Burnout { mount, delay_s } => Trigger::Burnout {
            // A cluster's tubes light together, so its first lit tube's burnout is the mount's, as
            // `Assembly::ignition_times_s` takes it.
            motor: assembly
                .motors
                .iter()
                .position(|motor| motor.mounted.mount == *mount && !motor.fails)
                .ok_or(SimError::Domain {
                    what: "count of lit motors in the mount a separation is timed from",
                    value: 0.0,
                })?,
            delay_s: *delay_s,
        },
        _ => {
            return Err(SimError::Domain {
                what: "kind of `.ork` separation trigger (one this version does not know)",
                value: f64::NAN,
            });
        }
    };
    let separation = Separation::new(trigger, staging.after_stage);
    // A split at a stage's first burnout drops the stage's other motors still burning, as
    // OpenRocket's does (ADR-172); no other split may.
    Ok(if staging.drops_burning {
        separation.dropping_burning()
    } else {
        separation
    })
}

/// The flight's separations for `stagings`, in the order they fire
/// ([`hpr_io::ork::MotorConfiguration::stagings`]), each as [`separation`] maps it; give them to
/// [`hpr_sim::Simulation::with_separations`].
///
/// # Errors
///
/// As [`separation`], for each.
pub fn separations<'a>(
    stagings: impl IntoIterator<Item = &'a Staging>,
    assembly: &Assembly,
) -> Result<Vec<Separation>, SimError> {
    stagings
        .into_iter()
        .map(|staging| separation(staging, assembly))
        .collect()
}

/// OpenRocket's drag coefficient for a parachute whose `<cd>` is `auto`, on the canopy's area
/// `π D²/4`, `D` its diameter: the OpenRocket technical documentation v13.05, §4.2.5, citing
/// S. F. Hoerner, *Fluid-Dynamic Drag* (1965), p. 13-23. OpenRocket 24.12 reads it back
/// (`validation/fixtures/ork/openrocket-events.json`).
pub const OPENROCKET_PARACHUTE_DRAG_COEFFICIENT: f64 = 0.8;

/// A `.ork` configuration's recovery devices as the flight flies them ([`recovery`]).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct Recovered {
    /// The devices that deploy, in file order, each named by its part's name (its id where the
    /// name is empty).
    pub devices: Vec<Device>,
    /// The devices that never deploy in this configuration, by name, with why.
    pub not_deployed: Vec<(String, NeverDeploys)>,
    /// How many devices at the end of `devices` HPR Sim added rather than read from the file: the
    /// tumbles [`separated_recovery`] adds ([`tumbling`]). Zero from [`recovery`].
    #[serde(default)]
    pub added: usize,
}

/// Why a `.ork`'s recovery device never deploys in a configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum NeverDeploys {
    /// Its event is `never`.
    SetToNever,
    /// It deploys at its stage's ejection charge, and no motor of that stage has one: each is
    /// plugged, or the file states no delay.
    NoEjectionCharge,
    /// It deploys at its stage's ejection charge, and no motor of that stage lights. OpenRocket
    /// 24.12 opens such a device on its own stage's charges alone, never another stage's, joined
    /// or apart, so it never opens there either
    /// ([ADR-168](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0168-pods-powered-flies.md)).
    NoLitMotor,
}

impl NeverDeploys {
    /// Why, in words for a note.
    #[must_use]
    pub const fn reason(self) -> &'static str {
        match self {
            Self::SetToNever => "the file sets it to never",
            Self::NoEjectionCharge => {
                "no motor of its stage has an ejection charge (plugged, or no delay given)"
            }
            Self::NoLitMotor => {
                "it opens at its stage's ejection charge, and no motor of its stage lights"
            }
        }
    }
}

/// A `.ork` configuration's recovery that [`recovery`] does not fly.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum RecoveryRefused {
    /// A parachute or streamer inside a part HPR Sim does not read, such as a pod.
    #[error("a {tag} at {at} is inside a `{inside}`, which HPR Sim does not read")]
    Unread {
        /// `parachute` or `streamer`.
        tag: String,
        /// Where it is in the file.
        at: String,
        /// The tag of the outermost part not read.
        inside: String,
    },
    /// A device that deploys at the separation of the stage below, where the flight has no such
    /// separation with nothing ahead of it left to burn: [`separated_recovery`] opens one at that
    /// split alone ([ADR-165](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md)).
    #[error(
        "`{name}` deploys at the lower stage's separation, which HPR Sim flies only when that stage \
         drops away with nothing ahead of it left to burn"
    )]
    AtSeparation {
        /// The device's name.
        name: String,
    },
    /// A deployment event HPR Sim does not know, or none.
    #[error("`{name}` deploys at `{word}`, an event HPR Sim does not know")]
    UnknownEvent {
        /// The device's name.
        name: String,
        /// The word the file wrote, empty where it wrote none.
        word: String,
    },
    /// A device the assembled rocket does not hold as a parachute or a streamer.
    #[error("the recovery device `{id}` is not a parachute or streamer of the assembled rocket")]
    NoPart {
        /// The device's id.
        id: String,
    },
    /// A number outside its domain, or one the file leaves out that the device needs.
    #[error("`{name}`'s {what} is {value}, outside its domain")]
    Domain {
        /// The device's name.
        name: String,
        /// What the number is.
        what: &'static str,
        /// The number, `NaN` where the file states none.
        value: f64,
    },
    /// A part of a separated rocket that can't tumble, which the flight needs it to
    /// ([`tumbling`]).
    #[error("`{name}` can't be flown: {why}")]
    Tumbling {
        /// The tumble's name, after its part's stage.
        name: String,
        /// Why the tumble's drag was refused, from the flight's error.
        why: String,
    },
    /// The part that keeps the nose, after a separation with nothing ahead of it left to burn,
    /// has no device of its own open by the split: the flight flies it on as a point with its
    /// devices' drag alone, which would be a coast with no drag at all ([`tumbling`],
    /// [ADR-165](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md)).
    #[error(
        "{name}, the part that keeps the nose, would coast with no drag from the separation at \
         {time_s:.3} s: with nothing ahead of it left to burn, HPR Sim flies it as a point with only \
         its own devices' drag, and none of them is open by then; set one of them to open at \
         the lower stage's separation, with no delay, for HPR Sim to fly it"
    )]
    Coasts {
        /// The part, after its first stage.
        name: String,
        /// When the separation fires, s after launch.
        time_s: f64,
    },
    /// The part's own numbers, such as a material with no surface density.
    #[error("`{name}`: {source}")]
    Design {
        /// The device's name.
        name: String,
        /// The error.
        source: DesignError,
    },
}

/// The flight's devices for the parachutes and streamers of `rocket`, as the `.ork` file's
/// `recovery` states them, in the configuration `assembly` holds assembled: each as OpenRocket
/// flies it (the OpenRocket technical documentation v13.05,
/// §4.2.5; [ADR-153](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-153-a-orks-recovery-flown-as-openrocket-flies-it-held-to-its-descents-2026-10-04), the decision to fly them so).
///
/// - **Drag.** A parachute's drag area is `C_D · π D²/4`, `D` its canopy diameter and `C_D` the
///   file's, or [`OPENROCKET_PARACHUTE_DRAG_COEFFICIENT`] for `auto`. A streamer's is a stated
///   `C_D` on its area `l · w`, or for `auto` OpenRocket's appendix C correlation
///   ([`StreamerModel::OpenRocket`]) from its length, width and fabric, for a strip no wider than
///   it is long ([`hpr_sim::recovery::MIN_STREAMER_ASPECT_RATIO`]).
/// - **Opening.** At once ([`hpr_sim::Inflation::Instant`]), the file's delay after its event.
/// - **Events.** `apogee` is [`Trigger::Apogee`]; `altitude` is [`Trigger::Altitude`], which
///   opens at apogee a device set above it, where OpenRocket's one probed run never opened it
///   ([ADR-056](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-056-a-ork-designs-recovery-and-separation-read-as-written-with-openrockets-words-measured-2026-09-21) §5, the `.ork` reader's probe); `ejection` is [`Trigger::MotorDelay`] of the motor of the device's stage whose
///   charge fires first (OpenRocket's "first ejection charge" of the stage); `launch` is [`Trigger::Time`] at the delay. `never`, and `ejection` with a
///   motor that has no charge, give no device but a [`Recovered::not_deployed`] entry.
///
/// Every device rides the first body. For a configuration that separates under power, use
/// [`separated_recovery`], which puts each on the part that carries it.
///
/// # Errors
///
/// [`RecoveryRefused`]: a device in a part HPR Sim does not read, one deployed at a separation or
/// by an unknown event, at the charge of a stage with no lit motor, a part not in `assembly`,
/// or a number outside its domain.
pub fn recovery(
    recovery: &Recovery,
    rocket: &hpr_design::Rocket,
    assembly: &Assembly,
) -> Result<Recovered, RecoveryRefused> {
    map_devices(recovery, rocket, assembly, &[])
}

/// The flight's devices for a `.ork` configuration that separates at `separations`, in the order
/// they fire ([`separations`] makes them): [`recovery`]'s mapping, each device on the part that
/// carries it,
/// with the tumbles [`tumbling`] adds so that each part has drag from the moment it flies alone
/// (the decision record on a powered separation in `hpr sim`,
/// [ADR-159](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0159-a-powered-separation-in-hpr-sim.md)).
///
/// A device in a stage aft of the first split rides the booster, body 1; one between the first
/// split and the second rides the part the second drops, body 2, and so on
/// ([`Separation::body_of`]); the rest ride the sustainer, body 0, which keeps the nose. A
/// device's event is its own part's: `apogee` is that part's own apogee, and `ejection` the charge
/// of a motor in the device's stage. `lowerstageseparation`, OpenRocket's "Lower stage
/// separation", opens on the trigger of the split right behind the device's stage, plus its
/// delay, when that split comes with nothing ahead of it left to burn
/// ([ADR-165](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md)).
///
/// # Errors
///
/// As [`recovery`], but for `lowerstageseparation` on such a split; [`tumbling`]'s.
pub fn separated_recovery(
    recovery: &Recovery,
    rocket: &hpr_design::Rocket,
    assembly: &Assembly,
    separations: &[Separation],
) -> Result<Recovered, RecoveryRefused> {
    let mut out = map_devices(recovery, rocket, assembly, separations)?;
    let read = out.devices.len();
    out.devices = tumbling(out.devices, rocket, assembly, separations)?;
    out.added = out.devices.len() - read;
    Ok(out)
}

/// `devices` with a tumble added for each part of `separations` that needs one: the flight flies
/// a separated part as a point with no airframe drag of its own, so each part must have a device
/// open from the moment it flies alone ([`hpr_sim::Simulation::with_separations`]).
///
/// - **Each dropped part,** body `k + 1` for separation `k` (the booster first), tumbles from its
///   split ([`DeviceDrag::tumbling_stages`] over its stages), until the first of its own devices,
///   in list order, opens: that device releases the tumble, so a part under its parachute has the
///   parachute's drag alone, as OpenRocket flies a descent (its technical documentation v13.05,
///   §4.2.5).
/// - **The sustainer,** body 0, flies on as a rigid body with its own aerodynamics. Only when no
///   device rides it is a tumble from its apogee added, so that it has a descent.
/// - **The part that keeps the nose after a lone separation with nothing ahead of it left to
///   burn** ([`Separation::powered_at`]) flies on as a point too, from the split. A device of its
///   own must be open by then, one whose trigger's time, known before the flight, is at or before
///   the split's: else it would coast with no drag, and it is refused rather than tumbled, as
///   nothing says how a part with no fins flies on its own ([ADR-165](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md)).
///
/// The tumbles go after `devices`, so each device keeps its index. Each is named after its part's
/// first stage, such as "Booster, tumbling", or, for a stage with no name, "sustainer",
/// "booster" or "stage 2" (counted from the nose's, 1).
///
/// # Errors
///
/// [`RecoveryRefused::Tumbling`] when [`DeviceDrag::tumbling_stages`] refuses a part's stages,
/// such as a part with tube fins, or more than eight fins in a set; [`RecoveryRefused::Coasts`]
/// when the part that keeps the nose would coast.
pub fn tumbling(
    mut devices: Vec<Device>,
    rocket: &hpr_design::Rocket,
    assembly: &Assembly,
    separations: &[Separation],
) -> Result<Vec<Device>, RecoveryRefused> {
    let stage_count = assembly.layout.stages.len();
    if let [separation] = separations
        && let Some(time_s) = separation.trigger.known_time_s(assembly).ok().flatten()
        && !separation.powered_at(assembly, time_s)
    {
        let open = devices
            .iter()
            .filter(|device| device.body == 0)
            .any(|device| {
                device
                    .trigger
                    .known_time_s(assembly)
                    .ok()
                    .flatten()
                    .is_some_and(|known_s| known_s + device.lag_s <= time_s)
            });
        if !open {
            let name = rocket
                .stages
                .first()
                .map(|stage| stage.name.trim())
                .filter(|name| !name.is_empty())
                .map_or_else(|| "stage 1".to_owned(), |name| format!("`{name}`"));
            return Err(RecoveryRefused::Coasts { name, time_s });
        }
    }
    for body in 0..=separations.len() {
        let Some(stages) = Separation::stages_of_body(separations, body, stage_count) else {
            continue;
        };
        let first = devices.iter().position(|device| device.body == body);
        if body == 0 && first.is_some() {
            continue;
        }
        let name = rocket
            .stages
            .get(stages.0)
            .map(|stage| stage.name.as_str())
            .filter(|name| !name.is_empty())
            .map_or_else(
                || match body {
                    0 => "sustainer".to_owned(),
                    1 => "booster".to_owned(),
                    _ => format!("stage {}", stages.0 + 1),
                },
                str::to_owned,
            );
        let name = format!("{name}, tumbling");
        let drag = DeviceDrag::tumbling_stages(assembly, stages).map_err(|error| {
            RecoveryRefused::Tumbling {
                name: name.clone(),
                why: error.to_string(),
            }
        })?;
        let device = if body == 0 {
            Device::new(name, drag, Trigger::Apogee)
        } else {
            // A dropped part's devices act only once it flies on its own, so a time of zero is
            // its split.
            let tumble = Device::new(name, drag, Trigger::Time { time_s: 0.0 }).on_body(body);
            match first {
                Some(first) => tumble.with_release_by(first),
                None => tumble,
            }
        };
        devices.push(device);
    }
    Ok(devices)
}

/// [`recovery`]'s mapping, each device on the part that carries it after `separations`, if any.
fn map_devices(
    recovery: &Recovery,
    rocket: &hpr_design::Rocket,
    assembly: &Assembly,
    separations: &[Separation],
) -> Result<Recovered, RecoveryRefused> {
    let configuration = assembly.configuration.as_str();
    // The `.ork` reader lights no motor at a separation (each ignition it reads is a launch, a
    // time or another motor's burnout), so none waits on one here.
    let ignitions_s = assembly.ignition_times_s(|_| None);
    if let Some(unread) = recovery.unread.first() {
        return Err(RecoveryRefused::Unread {
            tag: unread.tag.clone(),
            at: unread.at.clone(),
            inside: unread.inside.clone(),
        });
    }
    let mut out = Recovered {
        devices: Vec::new(),
        not_deployed: Vec::new(),
        added: 0,
    };
    for device in &recovery.devices {
        let (stage, part) = assembly
            .layout
            .find(&device.id)
            .map(|(_, placed)| (placed.stage, &placed.part))
            .ok_or_else(|| RecoveryRefused::NoPart {
                id: device.id.clone(),
            })?;
        let name = component_name(rocket, &device.id)
            .filter(|name| !name.is_empty())
            .unwrap_or(&device.id)
            .to_owned();
        let domain = |what: &'static str, value: f64| RecoveryRefused::Domain {
            name: name.clone(),
            what,
            value,
        };
        let deployment = device.deployment_in(configuration);
        let delay_s = deployment.delay_s.unwrap_or(0.0);
        if !(delay_s.is_finite() && delay_s >= 0.0) {
            return Err(domain("deployment delay, s", delay_s));
        }
        // Set for `lowerstageseparation`, whose trigger is a split's.
        let mut at_split = false;
        let trigger = match &deployment.event {
            Some(DeployEvent::Apogee) => Trigger::Apogee,
            Some(DeployEvent::Altitude) => {
                let height = deployment.altitude_m.unwrap_or(f64::NAN);
                if !(height.is_finite() && height > 0.0) {
                    return Err(domain("deployment height above the ground, m", height));
                }
                Trigger::Altitude {
                    height_above_ground_m: height,
                }
            }
            Some(DeployEvent::Launch) => Trigger::Time { time_s: delay_s },
            Some(DeployEvent::Ejection) => {
                // The stage's motors that light at a known time, each with when its charge
                // fires, if it has one: the first charge opens the device.
                let lit: Vec<(usize, Option<f64>)> = assembly
                    .motors
                    .iter()
                    .zip(&ignitions_s)
                    .enumerate()
                    .filter(|(_, (motor, _))| motor.stage == stage)
                    .filter_map(|(index, (motor, ignition_s))| {
                        let charge_s = match motor.mounted.delay {
                            Some(Delay::Seconds(delay_s)) => ignition_s.map(|ignition_s| {
                                ignition_s + motor.mounted.motor.burnout_time_s() + delay_s
                            }),
                            _ => None,
                        };
                        ignition_s.map(|_| (index, charge_s))
                    })
                    .collect();
                if lit.is_empty() {
                    out.not_deployed.push((name, NeverDeploys::NoLitMotor));
                    continue;
                }
                let first = lit
                    .iter()
                    .filter_map(|&(index, charge_s)| Some((index, charge_s?)))
                    .min_by(|a, b| a.1.total_cmp(&b.1));
                match first {
                    Some((motor, _)) => Trigger::MotorDelay { motor },
                    None => {
                        out.not_deployed
                            .push((name, NeverDeploys::NoEjectionCharge));
                        continue;
                    }
                }
            }
            Some(DeployEvent::Never) => {
                out.not_deployed.push((name, NeverDeploys::SetToNever));
                continue;
            }
            Some(DeployEvent::LowerStageSeparation) => {
                // OpenRocket's "Lower stage separation": the stage behind the device's comes
                // away. Its record opens a payload's parachute in the step after that split, so
                // the device takes the split's own trigger, and its delay as its lag. A
                // sustainer's split is left out: a device opening under power is not something
                // the flight flies.
                let split = separations
                    .iter()
                    .find(|separation| separation.after_stage == stage)
                    .filter(|separation| {
                        separation
                            .trigger
                            .known_time_s(assembly)
                            .ok()
                            .flatten()
                            .is_some_and(|time_s| !separation.powered_at(assembly, time_s))
                    });
                match split.map(|separation| separation.trigger) {
                    Some(trigger @ (Trigger::Time { .. } | Trigger::Burnout { .. })) => {
                        at_split = true;
                        trigger
                    }
                    _ => return Err(RecoveryRefused::AtSeparation { name }),
                }
            }
            Some(other) => {
                return Err(RecoveryRefused::UnknownEvent {
                    name,
                    word: other.as_str().to_owned(),
                });
            }
            None => {
                return Err(RecoveryRefused::UnknownEvent {
                    name,
                    word: String::new(),
                });
            }
        };
        let stated = match &device.cd {
            Some(Dimension::Stated { value }) => {
                if !(value.is_finite() && *value > 0.0) {
                    return Err(domain("drag coefficient", *value));
                }
                Some(*value)
            }
            // OpenRocket's own, which a file that states nothing gets too.
            Some(Dimension::Automatic { .. }) | None => None,
            Some(_) => {
                return Err(domain(
                    "drag coefficient (a kind HPR Sim does not know)",
                    f64::NAN,
                ));
            }
        };
        let drag = match (device.kind, part) {
            (DeviceKind::Parachute, Part::Parachute(parachute)) => {
                let diameter_m = parachute.diameter_m;
                if !(diameter_m.is_finite() && diameter_m > 0.0) {
                    return Err(domain("canopy diameter, m", diameter_m));
                }
                // A drag area, not a `DeviceDrag::Canopy`: OpenRocket flies any positive
                // coefficient the file states, where a canopy's stays within Knacke's range.
                let drag_coefficient = stated.unwrap_or(OPENROCKET_PARACHUTE_DRAG_COEFFICIENT);
                DeviceDrag::DragArea {
                    cd_s_m2: drag_coefficient * std::f64::consts::PI * diameter_m * diameter_m
                        / 4.0,
                }
            }
            (DeviceKind::Streamer, Part::Streamer(streamer)) => {
                let (length_m, width_m) = (streamer.length_m, streamer.width_m);
                for (what, value) in [
                    ("streamer length, m", length_m),
                    ("streamer width, m", width_m),
                ] {
                    if !(value.is_finite() && value > 0.0) {
                        return Err(domain(what, value));
                    }
                }
                match stated {
                    Some(cd) => DeviceDrag::DragArea {
                        cd_s_m2: cd * length_m * width_m,
                    },
                    // The correlation needs a strip, as the flight checks.
                    None if length_m / width_m < MIN_STREAMER_ASPECT_RATIO => {
                        return Err(domain(
                            "streamer aspect ratio, length over width",
                            length_m / width_m,
                        ));
                    }
                    None => DeviceDrag::Streamer {
                        length_m,
                        width_m,
                        surface_density_kg_m2: streamer
                            .material
                            .surface_kg_m2("streamer")
                            .map_err(|source| RecoveryRefused::Design {
                                name: name.clone(),
                                source,
                            })?,
                        model: StreamerModel::OpenRocket,
                    },
                }
            }
            _ => {
                return Err(RecoveryRefused::NoPart {
                    id: device.id.clone(),
                });
            }
        };
        // A time from launch holds its delay already; any other trigger, a split's included,
        // waits its delay after.
        let lag_s = if matches!(trigger, Trigger::Time { .. }) && !at_split {
            0.0
        } else {
            delay_s
        };
        // The part that carries it: the booster for a stage aft of the first split, and so on.
        let body = Separation::body_of(separations, stage);
        out.devices.push(
            Device::new(name, drag, trigger)
                .with_lag_s(lag_s)
                .on_body(body),
        );
    }
    Ok(out)
}

/// The name of the component with id `id` anywhere in `rocket`.
fn component_name<'a>(rocket: &'a hpr_design::Rocket, id: &str) -> Option<&'a str> {
    fn search<'a>(components: &'a [hpr_design::tree::Component], id: &str) -> Option<&'a str> {
        components.iter().find_map(|component| {
            if component.id == id {
                Some(component.name.as_str())
            } else {
                search(&component.children, id)
            }
        })
    }
    rocket
        .stages
        .iter()
        .find_map(|stage| search(&stage.components, id))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two stages, an Estes F15 from the bundled catalog in each: configuration `burn` drops the
    /// booster at its burnout and lights the sustainer there; `time` drops it 4 s after launch
    /// and lights the sustainer at 6 s.
    const ORK: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<openrocket version="1.10" creator="OpenRocket 24.12">
  <rocket><name>Two-stage</name>
    <motorconfiguration configid="burn"/><motorconfiguration configid="time"/>
    <subcomponents>
      <stage><name>Sustainer</name><id>upper</id><subcomponents>
        <nosecone><name>Nose</name><id>nose</id><length>0.15</length><thickness>0.002</thickness>
          <shape>ogive</shape><aftradius>0.0165</aftradius></nosecone>
        <bodytube><name>Sustainer</name><id>sustainer</id><length>0.4</length>
          <thickness>0.001</thickness><radius>0.0165</radius>
          <motormount><ignitionevent>automatic</ignitionevent><ignitiondelay>0.0</ignitiondelay>
            <overhang>0.0</overhang>
            <motor configid="burn"><type>single</type><manufacturer>Estes</manufacturer>
              <designation>F15</designation><diameter>0.029</diameter><length>0.114</length>
              <delay>6.0</delay></motor>
            <motor configid="time"><type>single</type><manufacturer>Estes</manufacturer>
              <designation>F15</designation><diameter>0.029</diameter><length>0.114</length>
              <delay>6.0</delay></motor>
            <ignitionconfiguration configid="time"><ignitionevent>launch</ignitionevent>
              <ignitiondelay>6.0</ignitiondelay></ignitionconfiguration>
          </motormount></bodytube></subcomponents></stage>
      <stage><name>Booster</name><id>lower</id>
        <separationevent>burnout</separationevent><separationdelay>0.0</separationdelay>
        <separationconfiguration configid="time"><separationevent>launch</separationevent>
          <separationdelay>4.0</separationdelay></separationconfiguration>
        <subcomponents>
        <bodytube><name>Booster</name><id>booster</id><length>0.3</length>
          <thickness>0.001</thickness><radius>0.0165</radius>
          <motormount><ignitionevent>automatic</ignitionevent><ignitiondelay>0.0</ignitiondelay>
            <overhang>0.0</overhang>
            <motor configid="burn"><type>single</type><manufacturer>Estes</manufacturer>
              <designation>F15</designation><diameter>0.029</diameter><length>0.114</length>
              <delay>0.0</delay></motor>
            <motor configid="time"><type>single</type><manufacturer>Estes</manufacturer>
              <designation>F15</designation><diameter>0.029</diameter><length>0.114</length>
              <delay>0.0</delay></motor>
          </motormount></bodytube></subcomponents></stage>
    </subcomponents></rocket>
</openrocket>"#;

    /// The configuration's staging and its assembly.
    fn read(id: &str) -> (Staging, Assembly) {
        let file = hpr_io::ork::read(ORK.as_bytes()).expect("a readable design");
        let design = hpr_io::ork::design(&file.value).value;
        let configuration = design
            .motors
            .configurations
            .iter()
            .find(|configuration| configuration.id == id)
            .expect("the configuration");
        let staging = configuration.staging.clone().expect("a separation");
        (staging, design.rocket.assemble(id).expect("flown"))
    }

    #[test]
    fn a_burnout_separation_names_the_booster_motor_by_its_index() {
        let (staging, assembly) = read("burn");
        let booster = assembly
            .motors
            .iter()
            .position(|motor| motor.mount == "booster")
            .expect("the booster's motor");
        assert_eq!(
            separation(&staging, &assembly).expect("maps"),
            Separation::new(
                Trigger::Burnout {
                    motor: booster,
                    delay_s: 0.0
                },
                0
            )
        );

        // A tube set to fail is passed over for the first that lights, as the design's own
        // ignition times take it.
        let mut failing = assembly.clone();
        let mut dud = failing.motors[booster].clone();
        dud.fails = true;
        failing.motors.insert(booster, dud);
        assert_eq!(
            separation(&staging, &failing).expect("maps").trigger,
            Trigger::Burnout {
                motor: booster + 1,
                delay_s: 0.0
            }
        );

        // A mount whose every tube fails, or another configuration's assembly with no motor in
        // that mount, is refused.
        let mut dead = assembly.clone();
        dead.motors[booster].fails = true;
        assert!(matches!(
            separation(&staging, &dead),
            Err(SimError::Domain {
                what: "count of lit motors in the mount a separation is timed from",
                ..
            })
        ));
        let mut other = assembly;
        other.motors.retain(|motor| motor.mount != "booster");
        let error = separation(&staging, &other).expect_err("no booster motor");
        assert!(
            matches!(
                error,
                SimError::Domain {
                    what: "count of lit motors in the mount a separation is timed from",
                    ..
                }
            ),
            "{error}"
        );
    }

    /// [`ORK`] with a parachute opening at apogee in each stage's tube where `chutes` says, read:
    /// configuration `id`'s recovery, rocket, assembly and separation.
    fn staged(
        id: &str,
        chutes: (bool, bool),
    ) -> (Recovery, hpr_design::Rocket, Assembly, Separation) {
        let chute = |name: &str| {
            format!(
                r#"<subcomponents><parachute><name>{name}</name><id>{name}</id>
                  <axialoffset method="top">0.05</axialoffset><packedlength>0.03</packedlength>
                  <packedradius>0.012</packedradius><cd>auto</cd>
                  <material type="surface" density="0.067">Ripstop nylon</material>
                  <deployevent>apogee</deployevent><deploydelay>0.0</deploydelay>
                  <diameter>0.3</diameter></parachute></subcomponents>"#
            )
        };
        let mut text = ORK.to_owned();
        // A tube's parts may come before its motor mount: the reader takes them in any order.
        for (wanted, tube, name) in [
            (chutes.0, "<id>sustainer</id>", "Upper chute"),
            (chutes.1, "<id>booster</id>", "Lower chute"),
        ] {
            if wanted {
                let at = text.find(tube).expect("the tube's text") + tube.len();
                text.insert_str(at, &chute(name));
            }
        }
        assert_eq!(
            text.matches("<parachute>").count(),
            usize::from(chutes.0) + usize::from(chutes.1)
        );
        let file = hpr_io::ork::read(text.as_bytes()).expect("a readable design");
        let design = hpr_io::ork::design(&file.value).value;
        let configuration = design
            .motors
            .configurations
            .iter()
            .find(|configuration| configuration.id == id)
            .expect("the configuration");
        let assembly = design.rocket.assemble(id).expect("flown");
        let split = separation(configuration.staging.as_ref().expect("staged"), &assembly)
            .expect("a separation");
        (design.recovery, design.rocket, assembly, split)
    }

    /// M4.5g1: each device rides the part its stage is in after the split, and the booster
    /// tumbles from the split until its own device opens; a sustainer with a device of its own
    /// gets no tumble.
    #[test]
    fn each_device_rides_its_part_and_the_booster_tumbles_until_its_own_opens() {
        let (recovery_, rocket, assembly, split) = staged("burn", (true, true));
        let flown = separated_recovery(&recovery_, &rocket, &assembly, &[split]).expect("maps");
        let tumble = DeviceDrag::tumbling_stages(&assembly, (1, 1)).expect("tumbles");
        let chute = DeviceDrag::DragArea {
            cd_s_m2: 0.8 * std::f64::consts::PI * 0.3 * 0.3 / 4.0,
        };
        assert_eq!(
            flown.devices,
            vec![
                Device::new("Upper chute", chute, Trigger::Apogee),
                Device::new("Lower chute", chute, Trigger::Apogee).on_body(1),
                Device::new("Booster, tumbling", tumble, Trigger::Time { time_s: 0.0 })
                    .on_body(1)
                    .with_release_by(1),
            ]
        );
        assert_eq!(flown.added, 1);
        // Without a separation every device rides the first body, as before, and hpr adds none.
        let whole = recovery(&recovery_, &rocket, &assembly).expect("maps");
        assert!(whole.devices.iter().all(|device| device.body == 0));
        assert_eq!((whole.devices.len(), whole.added), (2, 0));
    }

    /// A part with no device of the file's: the booster tumbles from the split to the ground, and
    /// the sustainer from its apogee. (`hpr sim`'s tests fly both.)
    #[test]
    fn a_part_with_no_device_tumbles() {
        let (recovery_, rocket, assembly, split) = staged("burn", (false, false));
        let flown = separated_recovery(&recovery_, &rocket, &assembly, &[split]).expect("maps");
        assert_eq!(flown.added, 2);
        assert_eq!(
            flown.devices,
            vec![
                Device::new(
                    "Sustainer, tumbling",
                    DeviceDrag::tumbling_stages(&assembly, (0, 0)).expect("tumbles"),
                    Trigger::Apogee,
                ),
                Device::new(
                    "Booster, tumbling",
                    DeviceDrag::tumbling_stages(&assembly, (1, 1)).expect("tumbles"),
                    Trigger::Time { time_s: 0.0 },
                )
                .on_body(1),
            ]
        );
        // A booster chute alone: the sustainer still tumbles from its apogee, and the booster's
        // tumble is released by its chute, the first device in the list.
        let (recovery_, rocket, assembly, split) = staged("burn", (false, true));
        let flown = separated_recovery(&recovery_, &rocket, &assembly, &[split]).expect("maps");
        assert_eq!(flown.added, 2);
        let described: Vec<(&str, usize, Option<usize>)> = flown
            .devices
            .iter()
            .map(|d| (d.name.as_str(), d.body, d.released_by))
            .collect();
        assert_eq!(
            described,
            [
                ("Lower chute", 1, None),
                ("Sustainer, tumbling", 0, None),
                ("Booster, tumbling", 1, Some(0)),
            ]
        );
    }

    /// [`ORK`] made a payload rocket: the sustainer holds no motor, the booster drops 2 s after its
    /// burnout in `burn` (4 s after launch in `time`), and the top stage's parachute opens at
    /// `upper`, `upper_delay_s` after, the booster's at apogee. Configuration `id`'s recovery,
    /// rocket, assembly and separation.
    fn payload(
        id: &str,
        (upper, upper_delay_s): (&str, f64),
    ) -> (Recovery, hpr_design::Rocket, Assembly, Separation) {
        let mut text = ORK.to_owned();
        let start = text.find("<motormount>").expect("the sustainer's mount");
        let end = text[start..].find("</motormount>").expect("its end") + start;
        text.replace_range(start..end + "</motormount>".len(), "");
        let text = text.replacen(
            "<separationdelay>0.0</separationdelay>",
            "<separationdelay>2.0</separationdelay>",
            1,
        );
        let chute = |name: &str, event: &str, delay_s: f64| {
            format!(
                r#"<subcomponents><parachute><name>{name}</name><id>{name}</id>
                  <axialoffset method="top">0.05</axialoffset><packedlength>0.03</packedlength>
                  <packedradius>0.012</packedradius><cd>auto</cd>
                  <material type="surface" density="0.067">Ripstop nylon</material>
                  <deployevent>{event}</deployevent><deploydelay>{delay_s}</deploydelay>
                  <diameter>0.3</diameter></parachute></subcomponents>"#
            )
        };
        let text = text
            .replacen(
                "<id>sustainer</id>",
                &format!(
                    "<id>sustainer</id>{}",
                    chute("Upper chute", upper, upper_delay_s)
                ),
                1,
            )
            .replacen(
                "<id>booster</id>",
                &format!("<id>booster</id>{}", chute("Lower chute", "apogee", 0.0)),
                1,
            );
        let file = hpr_io::ork::read(text.as_bytes()).expect("a readable design");
        let design = hpr_io::ork::design(&file.value).value;
        let configuration = design
            .motors
            .configurations
            .iter()
            .find(|configuration| configuration.id == id)
            .expect("the configuration");
        let assembly = design.rocket.assemble(id).expect("flown");
        assert_eq!(assembly.motors.len(), 1);
        let split = separation(configuration.staging.as_ref().expect("staged"), &assembly)
            .expect("a separation");
        (design.recovery, design.rocket, assembly, split)
    }

    /// M4.5g3: a separation with nothing ahead of it left to burn opens a parachute set to
    /// `lowerstageseparation` on the split's own trigger, its delay after, on the part that keeps
    /// the nose; that part must have a device open by the split, or it is refused, as it would
    /// coast with no drag (ADR-165). A sustainer's powered split, or none, still refuses the word.
    #[test]
    fn a_payload_opens_at_its_split_or_is_refused_as_coasting() {
        let (recovery_, rocket, assembly, split) = payload("burn", ("lowerstageseparation", 0.0));
        let booster = 0;
        assert_eq!(
            split.trigger,
            Trigger::Burnout {
                motor: booster,
                delay_s: 2.0
            }
        );
        let split_s = split.trigger.known_time_s(&assembly).unwrap().unwrap();
        assert!(!split.powered_at(&assembly, split_s));
        let flown = separated_recovery(&recovery_, &rocket, &assembly, &[split]).expect("maps");
        let described: Vec<(&str, usize, Trigger, Option<usize>)> = flown
            .devices
            .iter()
            .map(|d| (d.name.as_str(), d.body, d.trigger, d.released_by))
            .collect();
        assert_eq!(
            described,
            [
                ("Upper chute", 0, split.trigger, None),
                ("Lower chute", 1, Trigger::Apogee, None),
                (
                    "Booster, tumbling",
                    1,
                    Trigger::Time { time_s: 0.0 },
                    Some(1)
                ),
            ]
        );
        assert_eq!(flown.added, 1);
        assert_eq!(flown.devices[0].lag_s, 0.0);
        // A split at a time: the parachute opens at that time.
        let (recovery_, rocket, assembly, split) = payload("time", ("lowerstageseparation", 0.0));
        assert_eq!(split.trigger, Trigger::Time { time_s: 4.0 });
        let flown = separated_recovery(&recovery_, &rocket, &assembly, &[split]).expect("maps");
        assert_eq!(flown.devices[0].trigger, Trigger::Time { time_s: 4.0 });
        assert_eq!(flown.devices[0].lag_s, 0.0);

        // With a delay, on either split, the device takes the split's trigger and waits the delay
        // once, as its lag: flown, it opens the delay after the split, not twice that.
        for id in ["burn", "time"] {
            let (recovery_, rocket, assembly, split) = payload(id, ("lowerstageseparation", 1.0));
            let mapped = map_devices(&recovery_, &rocket, &assembly, &[split]).expect("maps");
            let upper = &mapped.devices[0];
            assert_eq!((upper.trigger, upper.lag_s), (split.trigger, 1.0), "{id}");
            // A streamer open from launch gives the payload drag from the split, so it flies.
            let mut devices = mapped.devices;
            devices.push(Device::new(
                "streamer",
                DeviceDrag::DragArea { cd_s_m2: 0.001 },
                Trigger::Time { time_s: 0.0 },
            ));
            let devices = tumbling(devices, &rocket, &assembly, &[split]).expect("flies");
            let result = hpr_sim::Simulation::new(
                &rocket,
                id,
                hpr_sim::Environment::standard(
                    hpr_core::geodesy::Geodetic::from_degrees(45.0, 0.0, 0.0).expect("a site"),
                )
                .expect("standard air"),
                hpr_sim::Rail::vertical(1.0),
                hpr_sim::FlightSettings::default(),
            )
            .expect("a simulation")
            .with_recovery(devices)
            .expect("devices it takes")
            .with_separations(vec![split])
            .expect("a split it takes")
            .run(&mut ())
            .expect("a flight");
            let split_s = result
                .event(hpr_sim::EventKind::Separation)
                .expect("split")
                .sample
                .time_s;
            let opened_s = result
                .bodies
                .iter()
                .find(|body| body.body == 0)
                .and_then(|body| body.event(hpr_sim::EventKind::Deployment(0)))
                .expect("opened on the payload")
                .sample
                .time_s;
            assert!(
                (opened_s - (split_s + 1.0)).abs() < 1e-9,
                "{id}: {opened_s} vs {split_s}"
            );
        }

        // Opening after the split, by its delay or at apogee, or not at all, leaves a coast with
        // no drag: refused, naming the part and when it would start.
        for upper in [
            ("lowerstageseparation", 0.25),
            ("apogee", 0.0),
            ("never", 0.0),
        ] {
            let (recovery_, rocket, assembly, split) = payload("burn", upper);
            let refused =
                separated_recovery(&recovery_, &rocket, &assembly, &[split]).expect_err("coasts");
            assert_eq!(
                refused,
                RecoveryRefused::Coasts {
                    name: "`Sustainer`".to_owned(),
                    time_s: split_s,
                },
                "{upper:?}"
            );
            assert!(
                refused
                    .to_string()
                    .starts_with("`Sustainer`, the part that keeps the nose, would coast"),
                "{refused}"
            );
        }
        // Open before the split, at a time on the stack, it has drag from the split.
        let (recovery_, rocket, assembly, split) = payload("burn", ("launch", split_s - 0.5));
        let flown = separated_recovery(&recovery_, &rocket, &assembly, &[split]).expect("maps");
        assert_eq!(
            flown.devices[0].trigger,
            Trigger::Time {
                time_s: split_s - 0.5
            }
        );

        // With no separation the word is refused, as before.
        let (recovery_, rocket, assembly, _) = payload("burn", ("lowerstageseparation", 0.0));
        assert_eq!(
            recovery(&recovery_, &rocket, &assembly).expect_err("refused"),
            RecoveryRefused::AtSeparation {
                name: "Upper chute".to_owned()
            }
        );
        // A sustainer's split, under power: refused too.
        let text = ORK.replacen(
            "<id>sustainer</id>",
            r#"<id>sustainer</id><subcomponents><parachute><name>Upper chute</name>
              <id>Upper chute</id><axialoffset method="top">0.05</axialoffset>
              <packedlength>0.03</packedlength><packedradius>0.012</packedradius><cd>auto</cd>
              <deployevent>lowerstageseparation</deployevent><deploydelay>0.0</deploydelay>
              <diameter>0.3</diameter></parachute></subcomponents>"#,
            1,
        );
        let file = hpr_io::ork::read(text.as_bytes()).expect("a readable design");
        let design = hpr_io::ork::design(&file.value).value;
        let staging = design.motors.configurations[0]
            .staging
            .clone()
            .expect("staged");
        let assembly = design.rocket.assemble("burn").expect("flown");
        let split = separation(&staging, &assembly).expect("a separation");
        assert!(split.powered_at(&assembly, staging.time_s));
        assert_eq!(
            separated_recovery(&design.recovery, &design.rocket, &assembly, &[split])
                .expect_err("refused"),
            RecoveryRefused::AtSeparation {
                name: "Upper chute".to_owned()
            }
        );
    }

    /// ADR-172: only a staging the `.ork` reader marks as dropping its stage's motors burning,
    /// at the stage's first burnout, lets the flight drop a burning motor; any other, such as one
    /// written by hand in a `.hpr`, keeps the flight's refusal.
    #[test]
    fn only_a_staging_marked_so_drops_a_burning_motor() {
        let (_, assembly) = read("time");
        let staging = |drops: bool| -> Staging {
            serde_json::from_value(serde_json::json!({
                "after_stage": 0,
                "trigger": { "time": { "time_s": 1.0 } },
                "time_s": 1.0,
                "drops_burning": drops,
            }))
            .expect("a staging")
        };
        let plain = Separation::new(Trigger::Time { time_s: 1.0 }, 0);
        assert_eq!(separation(&staging(false), &assembly).unwrap(), plain);
        assert_eq!(
            separation(&staging(true), &assembly).unwrap(),
            plain.dropping_burning()
        );
    }

    #[test]
    fn a_separation_at_a_time_stays_a_time() {
        let (staging, assembly) = read("time");
        assert_eq!(
            separation(&staging, &assembly).expect("maps"),
            Separation::new(Trigger::Time { time_s: 4.0 }, 0)
        );
    }

    /// One stage and an Estes F15 from the bundled catalog: configurations `charge` (a 6 s
    /// delay) and `plugged`. Its devices: a drogue `auto` at apogee 1 s late, a main of `C_D`
    /// 0.9 at 100 m, a streamer `auto` at the ejection charge, which `plugged` sets never to
    /// open by having no charge, a streamer of `C_D` 0.6 at launch plus 2 s, and a parachute set
    /// to `never`.
    const RECOVERY_ORK: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<openrocket version="1.10" creator="OpenRocket 24.12">
  <rocket><name>Recovery</name>
    <motorconfiguration configid="charge"/><motorconfiguration configid="plugged"/>
    <subcomponents>
      <stage><name>Sustainer</name><id>stage</id><subcomponents>
        <nosecone><name>Nose</name><id>nose</id><length>0.15</length><thickness>0.002</thickness>
          <shape>ogive</shape><aftradius>0.0165</aftradius></nosecone>
        <bodytube><name>Body</name><id>body</id><length>0.6</length>
          <thickness>0.001</thickness><radius>0.0165</radius>
          <subcomponents>
            <parachute><name>Drogue</name><id>drogue</id><axialoffset method="top">0.05</axialoffset><packedlength>0.03</packedlength>
              <packedradius>0.012</packedradius><cd>auto</cd>
              <material type="surface" density="0.067">Ripstop nylon</material>
              <deployevent>apogee</deployevent><deploydelay>1.0</deploydelay>
              <diameter>0.3</diameter><linecount>6</linecount><linelength>0.3</linelength>
              <linematerial type="line" density="0.0025">Nylon</linematerial></parachute>
            <parachute><name>Main</name><id>main</id><axialoffset method="top">0.05</axialoffset><packedlength>0.04</packedlength>
              <packedradius>0.012</packedradius><cd>0.9</cd>
              <material type="surface" density="0.067">Ripstop nylon</material>
              <deployevent>altitude</deployevent><deployaltitude>100.0</deployaltitude>
              <deploydelay>0.0</deploydelay>
              <diameter>0.6</diameter><linecount>8</linecount><linelength>0.5</linelength>
              <linematerial type="line" density="0.0025">Nylon</linematerial></parachute>
            <streamer><name>Charge streamer</name><id>charged</id><axialoffset method="top">0.05</axialoffset><packedlength>0.02</packedlength>
              <packedradius>0.01</packedradius><cd>auto</cd>
              <material type="surface" density="0.09">Mylar</material>
              <deployevent>ejection</deployevent><deploydelay>0.5</deploydelay>
              <striplength>1.2</striplength><stripwidth>0.1</stripwidth></streamer>
            <streamer><name></name><id>timed</id><axialoffset method="top">0.05</axialoffset><packedlength>0.02</packedlength>
              <packedradius>0.01</packedradius><cd>0.6</cd>
              <material type="surface" density="0.09">Mylar</material>
              <deployevent>launch</deployevent><deploydelay>2.0</deploydelay>
              <striplength>1.0</striplength><stripwidth>0.05</stripwidth></streamer>
            <parachute><name>Spare</name><id>spare</id><axialoffset method="top">0.05</axialoffset><packedlength>0.02</packedlength>
              <packedradius>0.01</packedradius><cd>auto</cd>
              <material type="surface" density="0.067">Ripstop nylon</material>
              <deployevent>never</deployevent><deploydelay>0.0</deploydelay>
              <diameter>0.3</diameter><linecount>6</linecount><linelength>0.3</linelength>
              <linematerial type="line" density="0.0025">Nylon</linematerial></parachute>
          </subcomponents>
          <motormount><ignitionevent>automatic</ignitionevent><ignitiondelay>0.0</ignitiondelay>
            <overhang>0.0</overhang>
            <motor configid="charge"><type>single</type><manufacturer>Estes</manufacturer>
              <designation>F15</designation><diameter>0.029</diameter><length>0.114</length>
              <delay>6.0</delay></motor>
            <motor configid="plugged"><type>single</type><manufacturer>Estes</manufacturer>
              <designation>F15</designation><diameter>0.029</diameter><length>0.114</length>
              <delay>none</delay></motor>
          </motormount></bodytube></subcomponents></stage>
    </subcomponents></rocket>
</openrocket>"#;

    /// `RECOVERY_ORK` with `edit` made to its text, read: its recovery, rocket and the assembly
    /// of configuration `id`.
    fn recovery_read(
        id: &str,
        edit: impl Fn(String) -> String,
    ) -> (Recovery, hpr_design::Rocket, Assembly) {
        let text = edit(RECOVERY_ORK.to_owned());
        let file = hpr_io::ork::read(text.as_bytes()).expect("a readable design");
        let design = hpr_io::ork::design(&file.value).value;
        let assembly = design.rocket.assemble(id).expect("flown");
        (design.recovery, design.rocket, assembly)
    }

    fn mapped(id: &str, edit: impl Fn(String) -> String) -> Result<Recovered, RecoveryRefused> {
        let (recovery_, rocket, assembly) = recovery_read(id, edit);
        recovery(&recovery_, &rocket, &assembly)
    }

    #[test]
    fn each_device_flies_as_openrocket_documents_it() {
        let flown = mapped("charge", |text| text).expect("maps");
        let motor = 0;
        let expected = vec![
            // `auto` on a parachute is 0.8 on the canopy's area, its delay a lag.
            Device::new(
                "Drogue",
                DeviceDrag::DragArea {
                    cd_s_m2: 0.8 * std::f64::consts::PI * 0.3 * 0.3 / 4.0,
                },
                Trigger::Apogee,
            )
            .with_lag_s(1.0),
            Device::new(
                "Main",
                DeviceDrag::DragArea {
                    cd_s_m2: 0.9 * std::f64::consts::PI * 0.6 * 0.6 / 4.0,
                },
                Trigger::Altitude {
                    height_above_ground_m: 100.0,
                },
            ),
            // `auto` on a streamer is appendix C's, from the strip and its fabric.
            Device::new(
                "Charge streamer",
                DeviceDrag::Streamer {
                    length_m: 1.2,
                    width_m: 0.1,
                    surface_density_kg_m2: 0.09,
                    model: StreamerModel::OpenRocket,
                },
                Trigger::MotorDelay { motor },
            )
            .with_lag_s(0.5),
            // A stated coefficient on the strip's area; a nameless part goes by its id; the
            // launch's delay is the trigger's time, not a lag.
            Device::new(
                "timed",
                DeviceDrag::DragArea {
                    cd_s_m2: 0.6 * 1.0 * 0.05,
                },
                Trigger::Time { time_s: 2.0 },
            ),
        ];
        assert_eq!(flown.devices, expected);
        assert_eq!(
            flown.not_deployed,
            vec![("Spare".to_owned(), NeverDeploys::SetToNever)]
        );
    }

    #[test]
    fn a_coefficient_above_knackes_range_flies_as_stated() {
        let flown = mapped("charge", |text| {
            text.replace("<cd>0.9</cd>", "<cd>2.5</cd>")
        })
        .expect("maps");
        assert_eq!(
            flown.devices[1].drag,
            DeviceDrag::DragArea {
                cd_s_m2: 2.5 * std::f64::consts::PI * 0.6 * 0.6 / 4.0
            }
        );
    }

    #[test]
    fn the_stages_first_charge_opens_the_device() {
        // A second motor in another mount of the same stage, its charge 2 s sooner: it opens
        // the streamer, though it is listed second.
        let (recovery_, rocket, mut assembly) = recovery_read("charge", |text| text);
        let mut sooner = assembly.motors[0].clone();
        sooner.mount = "second mount".to_owned();
        sooner.mounted.delay = Some(Delay::Seconds(4.0));
        assembly.motors.push(sooner);
        let streamer = |assembly: &Assembly| {
            recovery(&recovery_, &rocket, assembly)
                .expect("maps")
                .devices[2]
                .trigger
        };
        assert_eq!(streamer(&assembly), Trigger::MotorDelay { motor: 1 });
        // The listed-first motor plugged: the other's charge still opens it.
        assembly.motors[1].mounted.delay = Some(Delay::Seconds(8.0));
        assembly.motors[0].mounted.delay = Some(Delay::Plugged);
        assert_eq!(streamer(&assembly), Trigger::MotorDelay { motor: 1 });
        // A motor that never lights has no charge to give.
        assembly.motors[0].mounted.delay = Some(Delay::Seconds(1.0));
        assembly.motors[0].fails = true;
        assert_eq!(streamer(&assembly), Trigger::MotorDelay { motor: 1 });
    }

    #[test]
    fn a_pod_s_device_a_wide_streamer_and_a_missing_part_are_refused() {
        let (mut recovery_, rocket, assembly) = recovery_read("charge", |text| text);
        let mut unread = recovery_.clone();
        // The type is non-exhaustive, so it is built as a document would give it.
        unread.unread.push(
            serde_json::from_str(
                r#"{"at": "/rocket/podset/parachute", "tag": "parachute", "inside": "podset"}"#,
            )
            .expect("an unread device"),
        );
        assert!(matches!(
            recovery(&unread, &rocket, &assembly),
            Err(RecoveryRefused::Unread { inside, .. }) if inside == "podset"
        ));
        // A parachute the file calls a streamer is no part of the rocket's.
        recovery_.devices[0].kind = DeviceKind::Streamer;
        assert_eq!(
            recovery(&recovery_, &rocket, &assembly),
            Err(RecoveryRefused::NoPart {
                id: "drogue".to_owned()
            })
        );
        let wide = mapped("charge", |text| {
            text.replace(
                "<stripwidth>0.1</stripwidth>",
                "<stripwidth>1.5</stripwidth>",
            )
        });
        assert!(matches!(
            wide,
            Err(RecoveryRefused::Domain { name, what: "streamer aspect ratio, length over width", .. })
                if name == "Charge streamer"
        ));
    }

    #[test]
    fn a_plugged_motor_opens_nothing_at_its_charge() {
        let flown = mapped("plugged", |text| text).expect("maps");
        assert_eq!(
            flown
                .devices
                .iter()
                .map(|device| device.name.as_str())
                .collect::<Vec<_>>(),
            ["Drogue", "Main", "timed"]
        );
        assert_eq!(
            flown.not_deployed,
            vec![
                ("Charge streamer".to_owned(), NeverDeploys::NoEjectionCharge),
                ("Spare".to_owned(), NeverDeploys::SetToNever),
            ]
        );
    }

    #[test]
    fn a_configurations_own_deployment_wins() {
        let flown = mapped("charge", |text| {
            text.replace(
                "<deployevent>apogee</deployevent><deploydelay>1.0</deploydelay>",
                "<deployevent>apogee</deployevent><deploydelay>1.0</deploydelay>\
                 <deploymentconfiguration configid=\"charge\"><deployevent>altitude</deployevent>\
                 <deployaltitude>50.0</deployaltitude></deploymentconfiguration>",
            )
        })
        .expect("maps");
        // The event and height are the configuration's; the delay, which it leaves out, the
        // device's own.
        assert_eq!(
            flown.devices[0].trigger,
            Trigger::Altitude {
                height_above_ground_m: 50.0
            }
        );
        assert_eq!(flown.devices[0].lag_s, 1.0);
    }

    #[test]
    fn what_is_not_flown_is_refused_by_name() {
        let event = |word: &'static str| {
            move |text: String| {
                text.replace(
                    "<deployevent>apogee</deployevent>",
                    &format!("<deployevent>{word}</deployevent>"),
                )
            }
        };
        assert_eq!(
            mapped("charge", event("lowerstageseparation")),
            Err(RecoveryRefused::AtSeparation {
                name: "Drogue".to_owned()
            })
        );
        assert_eq!(
            mapped("charge", event("burnout")),
            Err(RecoveryRefused::UnknownEvent {
                name: "Drogue".to_owned(),
                word: "burnout".to_owned()
            })
        );
        // An altitude with no height, a negative delay, a coefficient of zero.
        let refused = |edit: &dyn Fn(String) -> String| match mapped("charge", edit) {
            Err(RecoveryRefused::Domain { name, what, .. }) => (name, what),
            other => panic!("not a domain error: {other:?}"),
        };
        assert_eq!(
            refused(&event("altitude")),
            ("Drogue".to_owned(), "deployment height above the ground, m")
        );
        assert_eq!(
            refused(&|text: String| text.replace(
                "<deploydelay>1.0</deploydelay>",
                "<deploydelay>-1.0</deploydelay>"
            )),
            ("Drogue".to_owned(), "deployment delay, s")
        );
        assert_eq!(
            refused(&|text: String| text.replace("<cd>0.9</cd>", "<cd>0.0</cd>")),
            ("Main".to_owned(), "drag coefficient")
        );
    }

    /// A device at its stage's charge, in a stage with no motor that lights, never opens, as in
    /// OpenRocket 24.12, which opens it on its own stage's charges alone; the others still fly
    /// (ADR-168).
    #[test]
    fn a_charge_with_no_motor_in_its_stage_never_opens() {
        let (recovery_, rocket, mut assembly) = recovery_read("charge", |text| text);
        let lit = recovery(&recovery_, &rocket, &assembly).unwrap();
        for motor in &mut assembly.motors {
            motor.fails = true;
        }
        let unlit = recovery(&recovery_, &rocket, &assembly).unwrap();
        let spare = ("Spare".to_owned(), NeverDeploys::SetToNever);
        assert_eq!(lit.not_deployed, std::slice::from_ref(&spare));
        assert_eq!(
            unlit.not_deployed,
            [
                ("Charge streamer".to_owned(), NeverDeploys::NoLitMotor),
                spare
            ]
        );
        assert_eq!(unlit.devices.len() + 1, lit.devices.len());
    }

    #[test]
    fn the_devices_fly() {
        // The mapping's devices are ones the flight takes, the charge's motor among them.
        let (recovery_, rocket, _) = recovery_read("charge", |text| text);
        let flown = recovery(
            &recovery_,
            &rocket,
            &rocket.assemble("charge").expect("flown"),
        )
        .expect("maps");
        let simulation = hpr_sim::Simulation::new(
            &rocket,
            "charge",
            hpr_sim::Environment::standard(
                hpr_core::geodesy::Geodetic::from_degrees(45.0, 0.0, 0.0).expect("a site"),
            )
            .expect("standard air"),
            hpr_sim::Rail::vertical(1.0),
            hpr_sim::FlightSettings::default(),
        )
        .expect("a simulation")
        .with_recovery(flown.devices)
        .expect("devices it takes");
        let result = simulation.run(&mut ()).expect("a flight");
        let deployed = result
            .events
            .iter()
            .filter(|event| matches!(event.kind, hpr_sim::EventKind::Deployment(_)))
            .count();
        assert_eq!(deployed, 4);
    }
}
