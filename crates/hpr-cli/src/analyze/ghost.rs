//! `hpr analyze --ghost`: the logs' trajectories written as JSON, CZML or glTF
//! (`hpr_flightdata::ghost`).

use std::path::Path;

use hpr::hpr_flightdata::ghost::{self, Ghost, Input, StatedPad};
use hpr::hpr_flightdata::log::FlightLog;
use hpr::hpr_flightdata::readings::Readings;

use super::AnalyzeArgs;
use crate::Failure;
use crate::sim::same_file;

/// A ghost file's format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    Json,
    Czml,
    Gltf,
}

impl Format {
    /// The format the path's extension names.
    fn of(path: &str) -> Option<Self> {
        let extension = Path::new(path).extension()?.to_str()?.to_ascii_lowercase();
        match extension.as_str() {
            "json" => Some(Self::Json),
            "czml" => Some(Self::Czml),
            "gltf" => Some(Self::Gltf),
            _ => None,
        }
    }

    /// How a person calls it.
    fn name(self) -> &'static str {
        match self {
            Self::Json => "JSON",
            Self::Czml => "CZML",
            Self::Gltf => "glTF",
        }
    }
}

/// Refuses a ghost path no format matches, or one this run reads, or a pad that isn't one, before
/// any log is read.
pub(super) fn check_paths(args: &AnalyzeArgs) -> Result<(), Failure> {
    let read: Vec<_> = std::iter::once(&args.log)
        .chain(&args.with)
        .chain(&args.mapping)
        .chain(&args.layout)
        .map(|path| same_file(path))
        .collect();
    for path in &args.ghost {
        let shown = crate::printable(path);
        if Format::of(path).is_none() {
            return Err(Failure::helped(
                format!("{shown}: a ghost file's extension picks its format"),
                "name it .json, .czml or .gltf".to_owned(),
            ));
        }
        if read.contains(&same_file(path)) {
            return Err(Failure::helped(
                format!("{shown}: this run reads that file, and --ghost would write over it"),
                "give --ghost a file of its own".to_owned(),
            ));
        }
    }
    stated_pad(args).map(|_| ())
}

/// The pad `--pad` states.
fn stated_pad(args: &AnalyzeArgs) -> Result<Option<StatedPad>, Failure> {
    let Some(text) = &args.pad else {
        return Ok(None);
    };
    let refused = || {
        Failure::helped(
            format!("--pad {}: not a place", crate::printable(text)),
            "give latitude,longitude in degrees, then the height in meters if you know it, as \
             39.39,-8.29,120"
                .to_owned(),
        )
    };
    let numbers = text
        .split(',')
        .map(|part| part.trim().parse::<f64>().map_err(|_| refused()))
        .collect::<Result<Vec<f64>, Failure>>()?;
    let (latitude_deg, longitude_deg, height_m) = match numbers.as_slice() {
        [lat, lon] => (*lat, *lon, None),
        [lat, lon, height] => (*lat, *lon, Some(*height)),
        _ => return Err(refused()),
    };
    if !(latitude_deg.abs() <= 90.0 && longitude_deg.abs() <= 180.0)
        || height_m
            .is_some_and(|height| height.is_nan() || height.abs() > ghost::PAD_HEIGHT_LIMIT_M)
    {
        return Err(refused());
    }
    Ok(Some(StatedPad {
        latitude_deg,
        longitude_deg,
        height_m,
    }))
}

/// A ghost that can't be laid out or drawn, as the caller reads it.
fn refused(error: ghost::GhostError) -> Failure {
    Failure::helped(
        crate::printable(&error.to_string()),
        "a ghost file needs a height or a GNSS track, and logs that share a liftoff; lay a log \
         with none out alone"
            .to_owned(),
    )
}

/// The bytes of `ghost` in `format`.
fn written(ghost: &Ghost, format: Format) -> Result<String, Failure> {
    match format {
        Format::Json => {
            let mut bytes = Vec::new();
            crate::write_json(&mut bytes, ghost).map_err(Failure::Output)?;
            String::from_utf8(bytes).map_err(|error| Failure::Output(std::io::Error::other(error)))
        }
        Format::Czml => ghost.to_czml().map_err(refused),
        Format::Gltf => ghost.to_gltf().map_err(refused),
    }
}

/// Writes each `--ghost` file; the lines to tell the caller, one a file.
pub(super) fn write(
    args: &AnalyzeArgs,
    logs: &[(&str, &FlightLog, &Readings)],
) -> Result<Vec<String>, Failure> {
    if args.ghost.is_empty() {
        return Ok(Vec::new());
    }
    let names: Vec<String> = logs
        .iter()
        .map(|(path, _, _)| {
            Path::new(path)
                .file_name()
                .map_or_else(|| (*path).to_owned(), |n| n.to_string_lossy().into_owned())
        })
        .collect();
    let inputs: Vec<Input<'_>> = logs
        .iter()
        .zip(&names)
        .map(|((_, log, read), name)| Input {
            name: name.as_str(),
            log,
            read,
        })
        .collect();
    let built = ghost::build(&inputs, stated_pad(args)?).map_err(refused)?;
    let mut lines = Vec::with_capacity(args.ghost.len());
    for path in &args.ghost {
        // Every path passed `check_paths` on the way in.
        let Some(format) = Format::of(path) else {
            continue;
        };
        let text = written(&built, format)?;
        std::fs::write(path, text).map_err(|error| crate::unwritable(path, &error))?;
        lines.push(format!(
            "wrote {}: a {} ghost file",
            crate::printable(path),
            format.name()
        ));
    }
    Ok(lines)
}
