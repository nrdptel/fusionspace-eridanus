//! A flight: a rocket launched from a rail in an environment, flown to the ground.

use std::sync::Arc;

use hpr_aero::{DragModel, DragTable};
use hpr_analysis::montecarlo::{DragOverride, FlightInputs};
use hpr_sim::metrics::{FlightMetrics, FlightSummary, Landing};
use hpr_sim::{
    EnvelopeFlag, FlightResult, FlightSettings, IssueWarning, Observer, Rail, Separation,
    Simulation,
};
use serde::{Deserialize, Serialize};

use crate::environment::Environment;
use crate::error::{Error, finite};
use crate::rocket::Rocket;

/// How a flight is launched: which rocket, where, and from what rail. Made by
/// [`Flight::builder`]; [`FlightBuilder::fly`] flies it.
#[derive(Debug, Clone)]
pub struct FlightBuilder<'a> {
    rocket: &'a Rocket,
    environment: &'a Environment,
    rail: Rail,
    /// The rail's angles, degrees, where set: they take the place of the rail's own.
    inclination_deg: Option<f64>,
    heading_deg: Option<f64>,
    settings: FlightSettings,
    /// A drag model flown in place of hpr's drag buildup, where set.
    drag_model: Option<Arc<dyn DragModel>>,
    /// A drag table flown in place of hpr's drag buildup, where set; never with a model.
    drag_table: Option<DragTable>,
    /// The stack coming apart in flight, in the order it does: none where unset.
    separations: Vec<Separation>,
}

impl FlightBuilder<'_> {
    /// The rail's angle above the horizon, degrees: 90, the default, is vertical, and 85 leans
    /// 5° off it. (OpenRocket's launch rod angle is measured from the vertical instead: its 5° is
    /// 85 here.)
    #[must_use]
    pub fn inclination_deg(mut self, inclination_deg: f64) -> Self {
        self.inclination_deg = Some(inclination_deg);
        self
    }

    /// The direction the rail leans toward, clockwise from true north, degrees: 0, the
    /// default, is north and 90 is east. A wind's direction is where it blows from, so a rail
    /// leaning into a west wind has both at 270. On a vertical rail the heading still turns the
    /// rocket about its axis, which way its fins face. That matters to a set of one or two fins,
    /// whose lift depends on which way the air meets them; three or more equal fins lift nearly
    /// the same whichever way they face.
    #[must_use]
    pub fn heading_deg(mut self, heading_deg: f64) -> Self {
        self.heading_deg = Some(heading_deg);
        self
    }

    /// The whole rail, in place of the one [`Flight::builder`] made: its length, its angles in
    /// radians, the rocket's roll on it and its friction ([`Rail`]).
    /// [`FlightBuilder::inclination_deg`] and [`FlightBuilder::heading_deg`], called before or
    /// after, still set its angles.
    #[must_use]
    pub fn rail(mut self, rail: Rail) -> Self {
        self.rail = rail;
        self
    }

    /// The integrator and its limits, in place of the defaults ([`FlightSettings`]).
    #[must_use]
    pub fn settings(mut self, settings: FlightSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Flies with `separation`: at its trigger the stack comes apart at a stage boundary, and
    /// each part flies on to its own landing ([`Simulation::with_separation`], which has the
    /// rules). Every part needs a recovery device on the rocket, each on its own part
    /// ([`hpr_sim::Device::on_body`]); for a `.ork` file, [`crate::ork::separated_recovery`]
    /// makes them. Each separated part's landing is in [`FlightSummary::body_landings`].
    #[must_use]
    pub fn separation(mut self, separation: Separation) -> Self {
        self.separations = vec![separation];
        self
    }

    /// Flies with `separations`, in the order they fire, each at a stage boundary further forward
    /// than the one before: a stack that drops its stages one at a time under power
    /// ([`Simulation::with_separations`], which has the rules). As [`FlightBuilder::separation`],
    /// every part needs a device of its own; for a `.ork` file, [`crate::ork::separations`] and
    /// [`crate::ork::separated_recovery`] make them. In place of any set before.
    #[must_use]
    pub fn separations(mut self, separations: Vec<Separation>) -> Self {
        self.separations = separations;
        self
    }

    /// Flies `model`'s drag in place of hpr's drag buildup: a model of your own, from a wind
    /// tunnel, another tool or your flights, or hpr's own adjusted
    /// ([`hpr_aero::custom`], [`Simulation::with_drag_model`]). The model gives the zero-lift
    /// drag coefficient on the rocket's reference area, by default a circle of its largest body
    /// diameter; unlike a drag table's, a model's number isn't rescaled, so a curve measured on
    /// another area is converted before it is returned. The normal force, center of pressure,
    /// roll and damping stay hpr's, so the stability margin a [`Rocket`] reports doesn't change.
    /// The last model or table set is the one flown ([`FlightBuilder::drag_table`]), and every
    /// flight of this builder shares it.
    ///
    /// ```
    /// # use hpr::rocket::{Fins, Mass, MotorTube, Nose, Tube, material};
    /// # use hpr::{Environment, FinPlanform, Flight, Motor, NoseShape, Position, Rocket};
    /// use hpr::hpr_aero::{AeroError, DragModel, DragQuery};
    ///
    /// /// hpr's own drag, 20% higher.
    /// #[derive(Debug)]
    /// struct Rougher;
    ///
    /// impl DragModel for Rougher {
    ///     fn zero_lift_drag(&self, query: &DragQuery<'_>) -> Result<f64, AeroError> {
    ///         Ok(1.2 * query.buildup()?.zero_lift_coefficient)
    ///     }
    /// }
    ///
    /// # let mut rocket = Rocket::new("Small", 0.0563)?;
    /// # rocket
    /// #     .add_nose(Nose::hollow(NoseShape::Ogive { radius_ratio: 1.0 }, 0.22, 0.0015, material("abs")?))?
    /// #     .add_tube(Tube::new(0.9, 0.00115, material("kraft_phenolic")?))?
    /// #     .add_fins(Fins::new(
    /// #         3,
    /// #         FinPlanform::Trapezoidal { root_chord_m: 0.1, tip_chord_m: 0.04, span_m: 0.045, sweep_m: 0.05 },
    /// #         0.003175,
    /// #         material("birch_plywood")?,
    /// #     ))?
    /// #     .add_motor_tube(MotorTube::new(0.2, 0.029, 0.001, material("kraft_phenolic")?))?
    /// #     .add_mass(Mass::new(0.2, Position::Top { aft_offset_m: 0.07 }))?
    /// #     .set_motor(Motor::from_catalog("H54")?)?;
    /// let environment = Environment::new(32.99, -106.97, 1400.0)?;
    /// let launch = Flight::builder(&rocket, &environment, 1.8);
    /// let own = launch.fly()?.apogee_m().ok_or("no apogee")?;
    /// let rougher = launch.drag_model(Rougher).fly()?.apogee_m().ok_or("no apogee")?;
    /// assert!(rougher < own);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn drag_model(self, model: impl DragModel + 'static) -> Self {
        self.shared_drag_model(Arc::new(model))
    }

    /// As [`FlightBuilder::drag_model`], with a model already shared: one model, a large table
    /// say, flown by many builders without a copy for each.
    #[must_use]
    pub fn shared_drag_model(mut self, model: Arc<dyn DragModel>) -> Self {
        self.drag_model = Some(model);
        self.drag_table = None;
        self
    }

    /// Flies `table`'s drag in place of hpr's drag buildup: another tool's zero-lift drag
    /// coefficient `C_D0` against Mach number, as RocketPy's `power_off_drag` and
    /// `power_on_drag` curves are ([`DragTable`], [`Simulation::with_drag_table`]). Its power-on
    /// curve, where it has one, is flown while a motor thrusts, and its power-off curve at every
    /// other time. A table read by [`DragTable::from_csv`] interpolates linearly between its Mach
    /// numbers and holds its end values past them; one built from [`hpr_core::interp::Table1D`]s
    /// interpolates and extrapolates as they say. A table on a reference diameter of its own
    /// ([`DragTable::with_reference_diameter_m`]) is rescaled to the rocket's reference area by
    /// the ratio of the two areas, `C_D0 · (d_table / d_rocket)²`. The normal force, center of
    /// pressure, roll and damping stay hpr's. The last model or table set is the one flown
    /// ([`FlightBuilder::drag_model`]). A flight that meets a negative coefficient in the table,
    /// drag that would push the rocket along, stops with an error that says so.
    ///
    /// ```
    /// # use hpr::rocket::{Fins, Mass, MotorTube, Nose, Tube, material};
    /// # use hpr::{Environment, FinPlanform, Flight, Motor, NoseShape, Position, Rocket};
    /// use hpr::hpr_aero::DragTable;
    ///
    /// # let mut rocket = Rocket::new("Small", 0.0563)?;
    /// # rocket
    /// #     .add_nose(Nose::hollow(NoseShape::Ogive { radius_ratio: 1.0 }, 0.22, 0.0015, material("abs")?))?
    /// #     .add_tube(Tube::new(0.9, 0.00115, material("kraft_phenolic")?))?
    /// #     .add_fins(Fins::new(
    /// #         3,
    /// #         FinPlanform::Trapezoidal { root_chord_m: 0.1, tip_chord_m: 0.04, span_m: 0.045, sweep_m: 0.05 },
    /// #         0.003175,
    /// #         material("birch_plywood")?,
    /// #     ))?
    /// #     .add_motor_tube(MotorTube::new(0.2, 0.029, 0.001, material("kraft_phenolic")?))?
    /// #     .add_mass(Mass::new(0.2, Position::Top { aft_offset_m: 0.07 }))?
    /// #     .set_motor(Motor::from_catalog("H54")?)?;
    /// let environment = Environment::new(32.99, -106.97, 1400.0)?;
    /// let launch = Flight::builder(&rocket, &environment, 1.8);
    /// // C_D0 of 0.45 up to Mach 0.8, 0.6 by Mach 1.2; 0.05 less while the motor burns.
    /// let table = DragTable::from_csv(
    ///     "mach,cd\n0,0.45\n0.8,0.45\n1.2,0.6\n",
    ///     Some("mach,cd\n0,0.40\n0.8,0.40\n1.2,0.55\n"),
    /// )?;
    /// let tabled = launch.clone().drag_table(table).fly()?.apogee_m().ok_or("no apogee")?;
    /// let own = launch.fly()?.apogee_m().ok_or("no apogee")?;
    /// assert_ne!(tabled, own);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn drag_table(mut self, table: DragTable) -> Self {
        self.drag_table = Some(table);
        self.drag_model = None;
        self
    }

    /// The [`Simulation`] [`FlightBuilder::fly`] runs, for what the facade doesn't offer: user
    /// events, staging, mass shifts and the rest of [`hpr_sim`].
    ///
    /// # Errors
    ///
    /// - [`Error::NoMotor`] for a rocket with no motor.
    /// - [`Error::Domain`] for an inclination outside `(0°, 90°]` or a heading that isn't
    ///   finite.
    /// - [`Error::Sim`] for what [`Simulation::new`], [`Simulation::with_recovery`] and
    ///   [`Simulation::with_separations`] refuse: a rail that isn't positive in length, a design
    ///   with errors in it ([`hpr_design::checks`]), a recovery device the flight can't fly, a
    ///   separation with a part that carries no device.
    pub fn simulation(&self) -> Result<Simulation, Error> {
        Ok(self.inputs()?.simulation()?)
    }

    /// Everything the flight is flown from, as [`hpr_analysis::montecarlo::FlightInputs`]: what a
    /// Monte Carlo run disperses ([`hpr_analysis::montecarlo`]).
    ///
    /// # Errors
    ///
    /// - [`Error::NoMotor`] for a rocket with no motor.
    /// - [`Error::Domain`] for an inclination outside `(0°, 90°]` or a heading that isn't
    ///   finite.
    /// - [`Error::Sim`] for a rail that isn't positive in length or whose friction is negative.
    pub fn inputs(&self) -> Result<FlightInputs, Error> {
        let configuration_id = self.rocket.configuration_id().ok_or(Error::NoMotor)?;
        let mut rail = self.rail;
        if let Some(inclination_deg) = self.inclination_deg {
            if !(inclination_deg > 0.0 && inclination_deg <= 90.0) {
                return Err(Error::Domain {
                    what: "rail inclination, degrees above the horizon",
                    value: inclination_deg,
                });
            }
            rail.elevation_rad = inclination_deg.to_radians();
        }
        if let Some(heading_deg) = self.heading_deg {
            rail.azimuth_rad = finite("rail heading, degrees", heading_deg)?.to_radians();
        }
        rail.validate()?;
        let mut inputs = FlightInputs::new(
            self.rocket.design().clone(),
            configuration_id,
            self.environment.sim().clone(),
            rail,
        );
        inputs.settings = self.settings;
        inputs.recovery = self.rocket.recovery().to_vec();
        // A builder holds one or neither: each setter clears the other.
        if let Some(model) = &self.drag_model {
            inputs.drag = Some(DragOverride::Model(Arc::clone(model)));
        }
        if let Some(table) = &self.drag_table {
            inputs.drag = Some(DragOverride::Table(table.clone()));
        }
        inputs.separations.clone_from(&self.separations);
        Ok(inputs)
    }

    /// Flies the rocket from the rail to the ground.
    ///
    /// # Errors
    ///
    /// As [`FlightBuilder::simulation`], and [`Error::Sim`] for a flight that fails on the way,
    /// such as a model asked about a flow it doesn't cover.
    pub fn fly(&self) -> Result<Flight, Error> {
        self.fly_with(&mut ())
    }

    /// Flies the rocket, showing `observer` every step of the flight as it goes: a
    /// [`hpr_sim::Recorder`] keeps a trajectory, and an [`Observer`] of your own can watch for
    /// anything.
    ///
    /// # Errors
    ///
    /// As [`FlightBuilder::fly`], and whatever `observer` returns.
    pub fn fly_with(&self, observer: &mut dyn Observer) -> Result<Flight, Error> {
        let simulation = self.simulation()?;
        let mut metrics = FlightMetrics::new();
        let result = simulation.run(&mut (&mut metrics, observer))?;
        let summary = metrics.summary(&result, self.environment.sim())?;
        // The drag issues are hpr's buildup's: a flight on a drag of its own meets none.
        let own_drag = self.drag_model.is_some() || self.drag_table.is_some();
        let mut issue_warnings = if own_drag {
            Vec::new()
        } else {
            let layout = &simulation.assembly().layout;
            let mut warnings = hpr_sim::issues::layout_issue_warnings(layout, summary.max_mach);
            warnings.extend(hpr_sim::issues::friction_issue_warnings(summary.max_mach));
            warnings
        };
        // A separated part's descent never takes the rocket's drag, so its issues stay with a
        // drag of the flight's own.
        issue_warnings.extend(hpr_sim::issues::separated_part_issue_warnings(
            &result,
            summary.max_mach,
        ));
        // The stability issues are hpr's normal force's (ADR-181): a drag of the flight's own
        // keeps them. Asking why the supersonic run stopped may build its table, so only a flight
        // the switches can matter to asks.
        let aero = simulation.aero();
        if aero.normal_force_table().is_none() {
            let supersonic = summary.max_mach.is_some_and(|mach| {
                mach.value.is_nan() || mach.value > hpr_sim::issues::SUPERSONIC_SWITCH_MACH
            });
            let fallback = if supersonic {
                aero.supersonic_fallback()
            } else {
                None
            };
            let layout = &simulation.assembly().layout;
            issue_warnings.extend(hpr_sim::issues::layout_stability_issue_warnings(
                layout,
                summary.max_mach,
            ));
            issue_warnings.extend(hpr_sim::issues::stability_issue_warnings(
                fallback.as_ref(),
                summary.min_static_margin_cal,
                summary.max_mach,
            ));
        }
        // The issues whose signs depend on the case (ADR-201) are the rocket's mass, its pods,
        // its damping stations and the wind's, never the drag's or the normal-force table's: a
        // flight on its own keeps them. The stations' issue builds the supersonic table only for
        // a flight past the earliest join. The mass's and the stations' are each vehicle's the
        // flight flew as a rigid body: the stack, then each sustainer, on its own top speed.
        let vehicles = simulation.flown_vehicles(&result)?;
        let masses: Vec<_> = vehicles
            .iter()
            .flat_map(hpr_sim::FlownVehicle::mass_samples)
            .collect();
        issue_warnings.extend(hpr_sim::issues::center_of_gravity_issue_warnings(
            &masses,
            summary.max_mach,
        ));
        // A sustainer's layout is a cut of the stack's, so the stack's pods are every pod.
        issue_warnings.extend(hpr_sim::issues::pod_issue_warnings(
            &simulation.assembly().layout,
            summary.max_mach,
        ));
        // A vehicle the watcher has no entry for takes the flight's top speed, which is at
        // least its own: the cautious side.
        let by_vehicle = metrics.max_mach_by_vehicle();
        let stations: Vec<_> = vehicles
            .iter()
            .enumerate()
            .map(|(index, vehicle)| {
                let top = by_vehicle.get(index).copied().unwrap_or(summary.max_mach);
                (vehicle.aero.as_ref(), top)
            })
            .collect();
        issue_warnings.extend(hpr_sim::issues::body_station_issue_warnings(
            &stations,
            summary.max_mach,
        ));
        // A wind is read by height above mean sea level: the site's ellipsoidal height less the
        // geoid's undulation, plus a height above the site.
        let environment = self.environment.sim();
        let ground_msl_m = environment.site().height_m - environment.geoid_undulation_m;
        let top_msl_m = ground_msl_m + hpr_sim::issues::highest_height_m(&result);
        issue_warnings.extend(hpr_sim::issues::wind_issue_warnings(
            &environment.wind.direction_turns(),
            [ground_msl_m, top_msl_m],
            summary.max_mach,
        ));
        Ok(Flight {
            result,
            summary,
            issue_warnings,
        })
    }
}

/// A flown flight: what happened, and its metrics.
///
/// Heights are the rocket's center of gravity's, above the launch site: the rocket stands on
/// the rail at the start, so the first height is not zero. Speeds are relative to the ground.
/// [`Flight::summary`] has every metric; the methods below are the ones most asked for. A flight
/// serializes and reads back as a record; one read back isn't flown again or checked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flight {
    result: FlightResult,
    summary: FlightSummary,
    /// The known issues it meets, in its drag, its stability margin and its path; none on a
    /// record saved before they were kept.
    #[serde(default)]
    issue_warnings: Vec<IssueWarning>,
}

impl Flight {
    /// A flight of `rocket` in `environment` from a vertical, frictionless rail `rail_length_m`
    /// long, measured from the rocket's aft end at the start to the rail's top. Set the rest on
    /// the builder; [`FlightBuilder::fly`] flies it.
    #[must_use]
    pub fn builder<'a>(
        rocket: &'a Rocket,
        environment: &'a Environment,
        rail_length_m: f64,
    ) -> FlightBuilder<'a> {
        FlightBuilder {
            rocket,
            environment,
            rail: Rail::vertical(rail_length_m),
            inclination_deg: None,
            heading_deg: None,
            settings: FlightSettings::default(),
            drag_model: None,
            drag_table: None,
            separations: Vec::new(),
        }
    }

    /// Every metric of the flight: apogee, peaks, stability margins, landings.
    #[must_use]
    pub fn summary(&self) -> &FlightSummary {
        &self.summary
    }

    /// The flight itself: its events, its end, and every body's.
    #[must_use]
    pub fn result(&self) -> &FlightResult {
        &self.result
    }

    /// The apogee's height above the launch site, m; `None` if the flight ended before it.
    #[must_use]
    pub fn apogee_m(&self) -> Option<f64> {
        self.summary
            .apogee
            .map(|apogee| apogee.height_above_ground_m)
    }

    /// When the apogee came, s after launch.
    #[must_use]
    pub fn apogee_time_s(&self) -> Option<f64> {
        self.summary.apogee.map(|apogee| apogee.time_s)
    }

    /// The top speed, m/s.
    #[must_use]
    pub fn max_speed_m_s(&self) -> Option<f64> {
        self.summary.max_speed_m_s.map(|peak| peak.value)
    }

    /// The top Mach number.
    #[must_use]
    pub fn max_mach(&self) -> Option<f64> {
        self.summary.max_mach.map(|peak| peak.value)
    }

    /// The speed as the rocket left the rail, m/s.
    #[must_use]
    pub fn rail_exit_speed_m_s(&self) -> Option<f64> {
        self.summary.rail_exit_speed_m_s.map(|peak| peak.value)
    }

    /// Where and how fast the rocket landed; `None` if it never reached the ground.
    #[must_use]
    pub fn landing(&self) -> Option<&Landing> {
        self.summary.landing.as_ref()
    }

    /// Where the flight's numbers deserve less trust: unstable under power (a static margin below
    /// zero while a motor burns, so its apogee is not a prediction), and where it goes past what
    /// hpr's numbers have been checked for: faster than the fastest validated flight, at a high
    /// angle of attack, outside the core band or beyond the envelope ([`hpr_sim::envelope`]).
    /// Empty for a flight that raises none.
    #[must_use]
    pub fn envelope_flags(&self) -> Vec<EnvelopeFlag> {
        self.summary.envelope_flags()
    }

    /// The known errors in hpr's drag, a separated part's drag, the stability margin and the
    /// flight's path this flight meets, each by its issue number, with the parts whose shape meets
    /// its condition and which way its numbers lean ([`hpr_sim::issues`]). A flight on a drag
    /// model or table of its own ([`FlightBuilder::drag_model`], [`FlightBuilder::drag_table`]),
    /// whose drag isn't hpr's, meets none of hpr's drag issues but keeps the separated parts'
    /// (#179, #354), the stability ones unless its model has a normal-force table of its own, and
    /// those whose signs depend on the case, which no table changes (#8, #106, #213, #219). Empty
    /// for a flight that meets none, and for a record saved before the warnings were kept.
    #[must_use]
    pub fn issue_warnings(&self) -> &[IssueWarning] {
        &self.issue_warnings
    }
}
