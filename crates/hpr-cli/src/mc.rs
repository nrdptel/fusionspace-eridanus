//! `hpr mc`: a design flown many times, each flight's inputs scattered at random about the
//! nominal flight's, and how far its apogee and landing spread.
//!
//! The run is the library's own: the design, motor and launch are read as `hpr sim` reads them
//! ([`crate::sim::Setup`]), [`hpr::FlightBuilder::inputs`] gives the nominal flight, a `.ork`'s
//! separations included, and [`MonteCarlo`] flies the samples, each input drawn from a normal
//! distribution about its nominal value ([`hpr::hpr_analysis::montecarlo`]). What this prints and
//! writes is what a program calling `MonteCarlo::run` with the same seed gets, number for number:
//! the samples are spread over the machine's threads, and a sample's numbers don't depend on
//! which thread flies it. A failed sample is counted and its reason printed, never dropped.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use hpr::hpr_analysis::ellipse::{Ellipse, Scatter};
use hpr::hpr_analysis::montecarlo::{Dispersion, FailedAt, MonteCarlo, Outcome, Run, Sample};
use hpr::hpr_analysis::statistics::Distribution;
use hpr::hpr_analysis::table::RunTable;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::output::{
    EllipseKind, Export, ExportFormat, McDispersion, McEllipse, McExportMeta, McFailed, McFailedAt,
    McFailure, McFlag, McLanding, McNominal, McRun, McSpread,
};
use crate::sim::{self, FlightArgs, Setup};
use crate::{Failure, Out};

/// The flights `--runs` flies when it isn't given.
pub const DEFAULT_RUNS: u64 = 100;

/// The most flights `--runs` takes: a run keeps every flight's summary, about a kilobyte each,
/// and 100,000 take minutes on a laptop.
pub const MAX_RUNS: u64 = 100_000;

/// `hpr mc`'s arguments.
#[derive(Debug, clap::Args)]
pub struct McArgs {
    /// The design, its motor and the nominal launch.
    #[command(flatten)]
    pub flight: FlightArgs,
    /// How many flights to fly, 1 to 100000
    #[arg(long, value_name = "N", default_value_t = DEFAULT_RUNS, value_parser = crate::typed::number::<u64>())]
    pub runs: u64,
    /// The run's seed: the same seed, design and options fly the same flights, bit for bit, on
    /// one platform
    #[arg(long, value_name = "N", default_value_t = 0, value_parser = crate::typed::number::<u64>())]
    pub seed: u64,
    /// One standard deviation of each stage's mass without motors, as a fraction of it (0.02 is
    /// 2%)
    #[arg(long, value_name = "FRACTION", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub mass_sd: f64,
    /// One standard deviation of each stage's center of mass along the axis, m
    #[arg(long, value_name = "M", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub cg_sd: f64,
    /// One standard deviation of the rocket's zero-lift drag coefficient, as a fraction of it
    #[arg(long, value_name = "FRACTION", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub drag_sd: f64,
    /// One standard deviation of each motor's total impulse, as a fraction of it; its propellant
    /// mass scales with it
    #[arg(long, value_name = "FRACTION", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub impulse_sd: f64,
    /// One standard deviation of each motor's burn time, as a fraction of it, at the same impulse
    #[arg(long, value_name = "FRACTION", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub burn_time_sd: f64,
    /// One standard deviation of each motor's ejection delay, s
    #[arg(long, value_name = "S", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub delay_sd: f64,
    /// One standard deviation of the wind's speed at every height, as a fraction of it
    #[arg(long, value_name = "FRACTION", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub wind_sd: f64,
    /// One standard deviation of the wind's direction, degrees, about --wind-from
    #[arg(long, value_name = "DEG", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub wind_from_sd: f64,
    /// One standard deviation of the rail's angle above the horizon, degrees
    #[arg(long, value_name = "DEG", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub inclination_sd: f64,
    /// One standard deviation of the rail's heading, degrees
    #[arg(long, value_name = "DEG", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub heading_sd: f64,
    /// One standard deviation of each recovery device's lag after its trigger, s
    #[arg(long, value_name = "S", default_value_t = 0.0, value_parser = crate::typed::number::<f64>())]
    pub deployment_lag_sd: f64,
    /// Write every flight's draw and outcome to this .csv file, one row a flight, and a
    /// .meta.json sidecar beside it
    #[arg(long, value_name = "FILE")]
    pub export: Option<String>,
}

impl McArgs {
    /// The dispersion as given, each standard deviation checked.
    fn given(&self) -> Result<McDispersion, Failure> {
        let given = McDispersion {
            mass_sd_fraction: self.mass_sd,
            cg_sd_m: self.cg_sd,
            drag_sd_fraction: self.drag_sd,
            impulse_sd_fraction: self.impulse_sd,
            burn_time_sd_fraction: self.burn_time_sd,
            delay_sd_s: self.delay_sd,
            wind_sd_fraction: self.wind_sd,
            wind_from_sd_deg: self.wind_from_sd,
            inclination_sd_deg: self.inclination_sd,
            heading_sd_deg: self.heading_sd,
            deployment_lag_sd_s: self.deployment_lag_sd,
        };
        for (option, value) in [
            ("--mass-sd", self.mass_sd),
            ("--cg-sd", self.cg_sd),
            ("--drag-sd", self.drag_sd),
            ("--impulse-sd", self.impulse_sd),
            ("--burn-time-sd", self.burn_time_sd),
            ("--delay-sd", self.delay_sd),
            ("--wind-sd", self.wind_sd),
            ("--wind-from-sd", self.wind_from_sd),
            ("--inclination-sd", self.inclination_sd),
            ("--heading-sd", self.heading_sd),
            ("--deployment-lag-sd", self.deployment_lag_sd),
        ] {
            if !(value.is_finite() && value >= 0.0) {
                return Err(Failure::Input(format!(
                    "{option} {value}: a standard deviation is a finite number, at least 0"
                )));
            }
        }
        Ok(given)
    }
}

/// The library's dispersion for `given`: the angles in radians.
pub fn dispersion(given: &McDispersion) -> Dispersion {
    Dispersion {
        dry_mass_sd_fraction: given.mass_sd_fraction,
        cg_sd_m: given.cg_sd_m,
        drag_sd_fraction: given.drag_sd_fraction,
        impulse_sd_fraction: given.impulse_sd_fraction,
        burn_time_sd_fraction: given.burn_time_sd_fraction,
        ejection_delay_sd_s: given.delay_sd_s,
        wind_speed_sd_fraction: given.wind_sd_fraction,
        wind_heading_sd_rad: given.wind_from_sd_deg.to_radians(),
        rail_elevation_sd_rad: given.inclination_sd_deg.to_radians(),
        rail_azimuth_sd_rad: given.heading_sd_deg.to_radians(),
        deployment_lag_sd_s: given.deployment_lag_sd_s,
    }
}

/// Runs `hpr mc`.
pub(crate) fn run(args: &McArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    // Everything that can be refused is refused before the flights: the export, the run's size,
    // the dispersion, the design, the motor, the site.
    let export = args.export.as_deref().map(export_paths).transpose()?;
    if let Some((csv, meta)) = &export {
        let inputs = sim::read_files(&args.flight);
        for path in [csv.as_str(), meta.as_str()] {
            if inputs.contains(&sim::same_file(path)) {
                return Err(Failure::Input(format!(
                    "{path}: this run reads that file, and --export would write over it"
                )));
            }
        }
    }
    if !(1..=MAX_RUNS).contains(&args.runs) {
        return Err(Failure::Input(format!(
            "--runs {}: a run flies 1 to {MAX_RUNS} flights",
            args.runs
        )));
    }
    let given = args.given()?;
    let setup = Setup::read(&args.flight)?;
    let builder = setup.builder(&args.flight);
    // The nominal flight, as `hpr sim` flies it: its figures and the issues it meets.
    let nominal = builder
        .fly()
        .map_err(|error| sim::flight_refused(&error, &args.flight))?;
    let monte_carlo = MonteCarlo::new(builder.inputs().map_err(sim::input)?, dispersion(&given))
        .map_err(|error| Failure::Input(error.to_string()))?;
    let run = fly(&monte_carlo, args.seed, args.runs, to);
    let motors = setup.motors()?;
    drop(builder);

    let analysis = |error: hpr::hpr_analysis::AnalysisError| Failure::Input(error.to_string());
    let apogee = run.apogee().map_err(analysis)?;
    let distance = run
        .distribution(|flight| flight.nose_landing().map(|landing| landing.distance_m))
        .map_err(analysis)?;
    let landing = landing(&run).map_err(analysis)?;
    let mut read = setup.read;
    // `hpr sim`'s notes on the flight's own numbers, said of the nominal flight; a scattered
    // flight may differ, and its figures are in the export.
    let recovery = sim::recovery_flown(
        &setup.devices,
        (setup.devices.len() - setup.added, &setup.separations),
        nominal.result(),
    );
    read.notes.extend(
        sim::flight_notes(&recovery, &nominal)
            .into_iter()
            .map(|note| format!("the nominal flight: {note}")),
    );
    if given == McDispersion::default() {
        read.notes.push(
            "no input is scattered, so every flight is the nominal one: give a standard \
             deviation, such as --impulse-sd 0.03, to scatter one"
                .to_owned(),
        );
    }
    let written = match &export {
        Some((csv, meta)) => {
            let text = RunTable::new(&run).csv().map_err(analysis)?;
            std::fs::write(csv, text).map_err(|error| Failure::Input(format!("{csv}: {error}")))?;
            let document = McExportMeta {
                file: sim::file_name(csv),
                design: sim::file_name(&args.flight.design),
                configuration: setup.configuration_label.clone(),
                seed: args.seed,
                runs: args.runs,
                dispersion: given,
                catalog_as_of: crate::motors::catalog_as_of()?,
                kind: crate::trust::Kind::Simulated,
                trust: crate::trust::runs(),
            };
            let mut text = Vec::new();
            crate::write_json(&mut text, &document)
                .and_then(|()| std::fs::write(meta, text))
                .map_err(|error| Failure::Input(format!("{meta}: {error}")))?;
            Some(Export {
                path: csv.clone(),
                format: ExportFormat::Csv,
                rows: run.samples.len(),
                meta: Some(meta.clone()),
            })
        }
        None => None,
    };
    let summary = nominal.summary();
    let document = McRun {
        kind: crate::trust::Kind::Simulated,
        trust: crate::trust::runs(),
        design: crate::output::SimDesign {
            file: sim::file_name(&args.flight.design),
            format: read.format,
            name: setup.design_name,
            configuration: setup.configuration,
            configuration_name: setup.configuration_name,
            configuration_label: setup.configuration_label,
            configurations: setup.configurations,
        },
        motors,
        launch: sim::launch(&args.flight),
        dispersion: given,
        seed: args.seed,
        runs: args.runs,
        flown: run.samples.len() - run.failed().count(),
        failed: failed(&run),
        nominal: McNominal {
            apogee_m: summary
                .apogee
                .as_ref()
                .map(|apogee| apogee.height_above_ground_m),
            landing_distance_m: summary.nose_landing().map(|l| l.distance_m),
            landing_east_m: summary.nose_landing().map(|l| l.east_m),
            landing_north_m: summary.nose_landing().map(|l| l.north_m),
        },
        apogee_m: spread(&apogee),
        landing_distance_m: spread(&distance),
        landing,
        flags: flags(&run),
        issues: sim::issues(&nominal),
        export: written,
        notes: read.notes,
        warnings: read.warnings,
    };
    to.emit(&document, |out, diagnostics| {
        crate::mc_text::print(&document, out, diagnostics)
    })
}

/// How often the progress line is drawn again.
const PROGRESS_TICK: Duration = Duration::from_millis(100);

/// Flies the run's samples on every core, as [`MonteCarlo::run_parallel`] does, the same flights
/// bit for bit, drawing its progress on standard error ([`crate::console::progress_line`]) once
/// it has gone on for `to.progress` and until it ends; never with `--json` or on a pipe.
fn fly(monte_carlo: &MonteCarlo, seed: u64, runs: u64, to: &mut Out<'_>) -> Run {
    let Some(after) = to.progress else {
        return monte_carlo.run_parallel(seed, runs);
    };
    let flown = AtomicU64::new(0);
    let started = Instant::now();
    let samples = std::thread::scope(|scope| {
        let (done, finished) = mpsc::channel();
        let flown = &flown;
        let worker = scope.spawn(move || {
            let samples: Vec<Sample> = (0..runs)
                .into_par_iter()
                .map(|index| {
                    let sample = monte_carlo.sample(seed, index);
                    flown.fetch_add(1, Ordering::Relaxed);
                    sample
                })
                .collect();
            // The receiver waits until this is sent; it can't be gone.
            let _ = done.send(());
            samples
        });
        loop {
            let elapsed = started.elapsed();
            if elapsed >= after {
                let line =
                    crate::console::progress_line(flown.load(Ordering::Relaxed), runs, elapsed);
                to.diagnostics.progress(&line);
            }
            match finished.recv_timeout(PROGRESS_TICK) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
        to.diagnostics.end_progress();
        // A panic on a flight's thread is rayon's to pass on, as `run_parallel` would.
        worker
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
    });
    Run { seed, samples }
}

/// The `--export` file and its sidecar, refused if it isn't a `.csv`, its folder is missing, or
/// it is a folder.
fn export_paths(path: &str) -> Result<(String, String), Failure> {
    let csv = Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("csv"));
    if !csv {
        return Err(Failure::Input(format!(
            "{path}: hpr mc --export writes a .csv file"
        )));
    }
    if let Some(folder) = Path::new(path).parent()
        && !folder.as_os_str().is_empty()
        && !folder.is_dir()
    {
        return Err(Failure::Input(format!(
            "{path}: there is no folder {}",
            folder.display()
        )));
    }
    let meta = sim::sidecar(path);
    for file in [path, meta.as_str()] {
        if Path::new(file).is_dir() {
            return Err(Failure::Input(format!(
                "{file}: a folder, so hpr mc --export can't write a file there"
            )));
        }
    }
    Ok((path.to_owned(), meta))
}

/// The failed flights, one entry for each distinct reason, in the order each first happened.
fn failed(run: &Run) -> McFailed {
    let mut reasons: Vec<McFailure> = Vec::new();
    for sample in run.failed() {
        let Outcome::Failed { at, reason } = &sample.outcome else {
            continue;
        };
        let at = match at {
            FailedAt::Inputs => McFailedAt::Inputs,
            FailedAt::Flight => McFailedAt::Flight,
        };
        match reasons
            .iter_mut()
            .find(|failure| failure.at == at && failure.reason == *reason)
        {
            Some(failure) => failure.count += 1,
            None => reasons.push(McFailure {
                at,
                reason: reason.clone(),
                count: 1,
                first_index: sample.index,
            }),
        }
    }
    McFailed {
        count: run.failed().count(),
        reasons,
    }
}

/// A distribution's usual numbers.
fn spread(distribution: &Distribution) -> McSpread {
    let summary = distribution.summary();
    McSpread {
        attempted: summary.attempted,
        count: summary.count,
        mean: summary.mean,
        standard_deviation: summary.standard_deviation,
        min: summary.min,
        p05: summary.p05,
        p50: summary.p50,
        p95: summary.p95,
        max: summary.max,
    }
}

/// The landings' center and ellipses: the scatter's 50% and 95%, then the next flight's 95%.
fn landing(run: &Run) -> Result<McLanding, hpr::hpr_analysis::AnalysisError> {
    let scatter = run.landing()?;
    let center = scatter.mean();
    let mut ellipses = Vec::new();
    for (kind, ellipse) in [
        (EllipseKind::Scatter, scatter.ellipse(0.5)?),
        (EllipseKind::Scatter, scatter.ellipse(0.95)?),
        (EllipseKind::NextFlight, scatter.prediction_ellipse(0.95)?),
    ] {
        if let Some(ellipse) = ellipse {
            ellipses.push(ellipse_out(kind, &ellipse, &scatter));
        }
    }
    Ok(McLanding {
        count: scatter.count(),
        center_east_m: center.map(|c| c[0]),
        center_north_m: center.map(|c| c[1]),
        ellipses,
    })
}

/// An ellipse as the output states it.
fn ellipse_out(kind: EllipseKind, ellipse: &Ellipse, scatter: &Scatter) -> McEllipse {
    McEllipse {
        kind,
        level: ellipse.level,
        center_east_m: ellipse.center_east_m,
        center_north_m: ellipse.center_north_m,
        semi_major_m: ellipse.semi_major_m,
        semi_minor_m: ellipse.semi_minor_m,
        major_heading_deg: ellipse.major_heading_rad.to_degrees(),
        inside: scatter.share_inside(ellipse).map_or(0.0, |share| share.low),
    }
}

/// Each envelope flag the flown samples raise, in the flags' order, with how many raise it.
fn flags(run: &Run) -> Vec<McFlag> {
    let mut flags: Vec<McFlag> = Vec::new();
    for summary in run.samples.iter().filter_map(|sample| sample.summary()) {
        for flag in summary.envelope_flags() {
            let flag = sim::flag_kind(&flag);
            match flags.iter_mut().find(|counted| counted.flag == flag) {
                Some(counted) => counted.flights += 1,
                None => flags.push(McFlag { flag, flights: 1 }),
            }
        }
    }
    flags.sort_by_key(|counted| counted.flag as u8);
    flags
}
