//! Writing a flight out: a [`Recorder`]'s rows as CSV or JSON (or Parquet, with the `parquet`
//! feature), and the center of mass's path with the landings as GeoJSON or KML for a map.
//!
//! Every function builds the file's contents (text, or bytes for Parquet) and returns them; none
//! writes a file (the core crates do no I/O). Text numbers are written in Rust's shortest
//! round-trip form and Parquet stores each `f64`'s own 8 bytes, so each reads back as the exact
//! value recorded. A value that isn't finite is refused rather than written as something a reader
//! would misread (`NaN`, or JSON's `null`).
//!
//! The two map formats put heights on different datums, as their specifications require:
//!
//! - **GeoJSON** (RFC 7946, section 4): longitude, latitude and height in meters above the WGS 84
//!   ellipsoid.
//! - **KML** (OGC 07-147r2, `altitudeMode` `absolute`): longitude, latitude and height above mean
//!   sea level (KML's geoid is EGM96), which is the ellipsoidal height less the site's geoid
//!   undulation ([`Environment::geoid_undulation_m`]). The undulation is taken as constant over the
//!   flight; the geoid's slope, about 5 cm/km and up to some 30 cm/km in mountains, moves it by
//!   centimeters to decimeters over a rocket's few kilometers.
//!
//! Neither cuts a path that crosses the antimeridian (±180° longitude), which RFC 7946 section 3.1.9
//! asks for; a flight there draws a line the long way round the globe.
//!
//! Each file names the program that wrote it, its version and its designation in the FusionSpace
//! product system ([`hpr_core::tool`]), as a drawing's title block names the tool that made it:
//! JSON and GeoJSON in a top-level `tool` object, `{"name", "version", "designation"}` (in
//! GeoJSON a foreign member, which RFC 7946 section 6.1 allows); KML in an XML comment after the
//! declaration; Parquet in its key-value metadata. CSV is left as bare rows, so it opens cleanly
//! in a spreadsheet: a program writing one keeps the stamp beside it.
//!
//! The two text formats follow the FusionSpace product system's `data.md` (*Files and exports*,
//! *Copying and typing numbers*): a CSV's header gives each column in words with its unit in
//! brackets, `time [s]` ([`csv_header`]), where JSON and Parquet keep the unit in the name,
//! `time_s`; and JSON and GeoJSON text is ASCII, any other character written as a `\u` escape
//! ([`ascii`]), so the designation's middle dot reads back as itself.

use hpr_core::DVec3;
use serde_json::{Value, json};

use crate::environment::Environment;
use crate::error::SimError;
use crate::metrics::{FlightSummary, Landing};
use crate::recorder::Recorder;

#[cfg(feature = "parquet")]
mod parquet;
#[cfg(feature = "parquet")]
pub use parquet::{PARQUET_PAGE_ROWS, parquet, parquet_with};

/// The `tool` object a JSON or GeoJSON export opens with: the program's name, version and
/// designation, from [`hpr_core::tool`].
#[derive(serde::Serialize)]
struct Tool {
    name: &'static str,
    version: &'static str,
    designation: &'static str,
}

const TOOL: Tool = Tool {
    name: hpr_core::tool::NAME,
    version: hpr_core::tool::VERSION,
    designation: hpr_core::tool::DESIGNATION,
};

/// A JSON export of the recorded rows, its keys in this order: the caller's own fields come
/// between the program and the rows.
#[derive(serde::Serialize)]
struct Rows<'a, T: serde::Serialize> {
    tool: Tool,
    #[serde(flatten)]
    about: &'a T,
    columns: &'a [String],
    rows: &'a [Vec<f64>],
}

/// A GeoJSON `FeatureCollection`, its keys in this order: `tool` is a foreign member.
#[derive(serde::Serialize)]
struct FeatureCollection<'a, T: serde::Serialize> {
    #[serde(rename = "type")]
    kind: &'static str,
    tool: Tool,
    #[serde(flatten)]
    about: &'a T,
    features: Vec<Value>,
}

/// `value` as compact JSON text, in ASCII ([`ascii`]).
fn to_text(value: &impl serde::Serialize) -> Result<String, SimError> {
    // Every value here is finite (checked before) and every key a string, the only two things
    // `serde_json` refuses; the error is mapped rather than unwrapped all the same.
    serde_json::to_string(value)
        .map(|text| ascii(&text))
        .map_err(|_| SimError::Unsupported {
            what: "a value JSON can't hold",
        })
}

/// JSON text in ASCII: every character outside it written as its `\u` escape, a character past
/// U+FFFF as its UTF-16 surrogate pair (RFC 8259, section 7), so `·` (U+00B7) becomes `\u00b7`.
/// The product system's `data.md` asks JSON to be ASCII (*Copying and typing numbers*). A reader
/// gets back the same strings: outside a string JSON text is ASCII already, and inside one an
/// escape stands for its character.
#[must_use]
pub fn ascii(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    for c in json.chars() {
        if c.is_ascii() {
            out.push(c);
        } else {
            let mut units = [0_u16; 2];
            for unit in c.encode_utf16(&mut units) {
                out.push_str(&format!("\\u{unit:04x}"));
            }
        }
    }
    out
}

/// The units a column's name can end in, as its last one or two `_`-separated words, and how a
/// CSV header writes each, in ASCII: two-word units first, so `m_s` is meters per second and not
/// seconds.
const UNITS: [(&[&str], &str); 12] = [
    (&["m", "s2"], "m/s^2"),
    (&["m", "s"], "m/s"),
    (&["rad", "s"], "rad/s"),
    (&["m2"], "m^2"),
    (&["pa"], "Pa"),
    (&["kg"], "kg"),
    (&["n"], "N"),
    (&["m"], "m"),
    (&["s"], "s"),
    (&["rad"], "rad"),
    (&["deg"], "deg"),
    (&["cal"], "cal"),
];

/// A column's name as a CSV header writes it: the name's words with spaces, then its unit in
/// brackets, as the product system's `data.md` writes a header (`time [s]`). The unit is the
/// name's last word or two, read from a table of twelve units: `time_s` is `time [s]`,
/// `velocity_east_m_s` is `velocity east [m/s]`, `max_acceleration_m_s2` is
/// `max acceleration [m/s^2]` and `dynamic_pressure_pa` is `dynamic pressure [Pa]`; a name with
/// no unit, such as `mach` or `drag_scale`, is its words alone. JSON and Parquet keep the name.
#[must_use]
pub fn csv_header(column: &str) -> String {
    let words: Vec<&str> = column.split('_').collect();
    for (unit, written) in UNITS {
        if words.len() > unit.len() && words.ends_with(unit) {
            let name = words[..words.len() - unit.len()].join(" ");
            return format!("{name} [{written}]");
        }
    }
    words.join(" ")
}

/// One point of the center of mass's path, placed on the Earth.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrackPoint {
    /// Time since launch, s.
    pub time_s: f64,
    /// Geodetic latitude, degrees north (WGS 84).
    pub latitude_deg: f64,
    /// Longitude, degrees east.
    pub longitude_deg: f64,
    /// Height above the WGS 84 ellipsoid, m.
    pub height_above_ellipsoid_m: f64,
    /// Height above mean sea level, m: the ellipsoidal height less the site's geoid undulation.
    pub height_above_sea_level_m: f64,
}

fn finite(what: &'static str, value: f64) -> Result<f64, SimError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SimError::Domain { what, value })
    }
}

/// The recorded rows as CSV (RFC 4180): a header of the columns in words with their units in
/// brackets ([`csv_header`]: `time [s]` for the column `time_s`), then one line per row, lines
/// ending in CRLF.
///
/// # Errors
///
/// [`SimError::Domain`] for a value that isn't finite.
pub fn csv(recorder: &Recorder) -> Result<String, SimError> {
    let header: Vec<String> = recorder.columns().iter().map(|c| csv_header(c)).collect();
    let mut out = header.join(",");
    out.push_str("\r\n");
    for row in recorder.rows() {
        let mut cells = Vec::with_capacity(row.len());
        for &value in row {
            cells.push(format!("{:?}", finite("recorded value", value)?));
        }
        out.push_str(&cells.join(","));
        out.push_str("\r\n");
    }
    Ok(out)
}

/// The recorded rows as JSON: `{"tool": {...}, "columns": [names], "rows": [[values], ...]}`,
/// each row in the columns' order. `tool` names the program that wrote the file:
/// `{"name": "FusionSpace HPR", "version": "0.1.0", "designation": "FS-ACHERNAR \u00b7 SW \u00b7
/// TOOL 001"}`, the text in ASCII ([`ascii`]). A [`FlightSummary`] is serializable on its own
/// (`serde_json::to_string`).
///
/// # Errors
///
/// [`SimError::Domain`] for a value that isn't finite.
pub fn json(recorder: &Recorder) -> Result<String, SimError> {
    json_with(recorder, &serde_json::Map::new())
}

/// The recorded rows as [`json()`] writes them, with `about`'s fields, in its order, between
/// `tool` and `columns`: what the caller knows of the flight that the recorder doesn't, such as
/// the design flown or how far to trust it. A struct or a [`serde_json::Map`] keeps the order
/// from one run to the next; a `HashMap` doesn't, so the same flight could write other bytes.
///
/// # Errors
///
/// [`SimError::Domain`] for a value that isn't finite; [`SimError::Unsupported`] for an `about`
/// that isn't a JSON object, or has a field named `tool`, `columns` or `rows`, which would write a
/// key twice.
pub fn json_with<T: serde::Serialize>(recorder: &Recorder, about: &T) -> Result<String, SimError> {
    let Ok(Value::Object(fields)) = serde_json::to_value(about) else {
        return Err(SimError::Unsupported {
            what: "fields beside the recording that aren't a JSON object",
        });
    };
    if ["tool", "columns", "rows"]
        .iter()
        .any(|key| fields.contains_key(*key))
    {
        return Err(SimError::Unsupported {
            what: "a field named tool, columns or rows beside the recording's own",
        });
    }
    for row in recorder.rows() {
        for &value in row {
            finite("recorded value", value)?;
        }
    }
    to_text(&Rows {
        tool: TOOL,
        about,
        columns: &recorder.columns(),
        rows: recorder.rows(),
    })
}

/// The center of mass's path from a recorder that kept [`crate::Channel::Time`] and
/// [`crate::Channel::CgPosition`], placed on `environment`'s ellipsoid.
///
/// # Errors
///
/// [`SimError::Unsupported`] if the recorder lacks either channel; [`SimError::Core`] if a
/// position has no geodetic coordinates.
pub fn track(recorder: &Recorder, environment: &Environment) -> Result<Vec<TrackPoint>, SimError> {
    let columns = recorder.columns();
    let index = |name: &str| columns.iter().position(|c| c == name);
    let (Some(t), Some(e), Some(n), Some(u)) = (
        index("time_s"),
        index("cg_east_m"),
        index("cg_north_m"),
        index("cg_up_m"),
    ) else {
        return Err(SimError::Unsupported {
            what: "a track from a recorder without the time and CG position channels",
        });
    };
    let frame = environment.earth.frame();
    recorder
        .rows()
        .iter()
        .map(|row| {
            let place = frame.geodetic_from_enu(DVec3::new(row[e], row[n], row[u]))?;
            Ok(TrackPoint {
                time_s: row[t],
                latitude_deg: place.latitude_rad.to_degrees(),
                longitude_deg: place.longitude_rad.to_degrees(),
                height_above_ellipsoid_m: place.height_m,
                height_above_sea_level_m: place.height_m - environment.geoid_undulation_m,
            })
        })
        .collect()
}

/// The flight's landing, then each separated body's, as `(label, landing)`.
fn landings(summary: &FlightSummary) -> Vec<(String, &Landing)> {
    let flight = summary.landing.iter().map(|l| ("landing".to_owned(), l));
    let bodies = summary.body_landings.iter().map(|l| {
        let label = match l.body {
            Some(body) => format!("body {body} landing"),
            None => "landing".to_owned(),
        };
        (label, l)
    });
    flight.chain(bodies).collect()
}

fn too_short(track: &[TrackPoint]) -> Result<(), SimError> {
    if track.len() < 2 {
        return Err(SimError::Unsupported {
            what: "a path of fewer than two points (a line needs two)",
        });
    }
    Ok(())
}

/// The path and the landings as a GeoJSON `FeatureCollection` (RFC 7946): a `LineString` of
/// `[longitude, latitude, height above the ellipsoid]` whose `properties.time_s` lists each
/// point's time, and a `Point` per landing of `summary` with its time, body, distance and descent
/// rate. A top-level `tool` object names the program that wrote it, as [`json()`]'s does: a
/// foreign member, which RFC 7946 section 6.1 allows and readers pass over.
///
/// # Errors
///
/// [`SimError::Unsupported`] for a track of fewer than two points (a `LineString` needs two);
/// [`SimError::Domain`] for a coordinate that isn't finite.
pub fn geojson(track: &[TrackPoint], summary: &FlightSummary) -> Result<String, SimError> {
    geojson_with(track, summary, &serde_json::Map::new())
}

/// The path and the landings as [`geojson()`] writes them, with `about`'s fields, in its order,
/// between `tool` and `features`: more foreign members (RFC 7946 section 6.1), such as the motor
/// catalog's date or how far to trust the landing point, as [`json_with`] carries them. A struct
/// or a [`serde_json::Map`] keeps the order from one run to the next.
///
/// # Errors
///
/// As [`geojson()`]; also [`SimError::Unsupported`] for an `about` that isn't a JSON object, or
/// has a field named `type`, `tool` or `features`, which would write a key twice, or one RFC 7946
/// section 7.1 keeps from a `FeatureCollection` (`geometry`, `properties`, `coordinates`,
/// `geometries`).
pub fn geojson_with<T: serde::Serialize>(
    track: &[TrackPoint],
    summary: &FlightSummary,
    about: &T,
) -> Result<String, SimError> {
    let Ok(Value::Object(fields)) = serde_json::to_value(about) else {
        return Err(SimError::Unsupported {
            what: "fields beside the map that aren't a JSON object",
        });
    };
    if [
        "type",
        "tool",
        "features",
        "geometry",
        "properties",
        "coordinates",
        "geometries",
    ]
    .iter()
    .any(|key| fields.contains_key(*key))
    {
        return Err(SimError::Unsupported {
            what: "a field beside the map named as a GeoJSON member",
        });
    }
    too_short(track)?;
    let mut coordinates = Vec::with_capacity(track.len());
    let mut times = Vec::with_capacity(track.len());
    for p in track {
        coordinates.push(json!([
            finite("longitude", p.longitude_deg)?,
            finite("latitude", p.latitude_deg)?,
            finite("height", p.height_above_ellipsoid_m)?,
        ]));
        times.push(finite("time", p.time_s)?);
    }
    let mut features = vec![json!({
        "type": "Feature",
        "geometry": {"type": "LineString", "coordinates": coordinates},
        "properties": {"name": "flight path", "time_s": times},
    })];
    for (label, l) in landings(summary) {
        features.push(json!({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [
                finite("longitude", l.longitude_deg)?,
                finite("latitude", l.latitude_deg)?,
            ]},
            "properties": {
                "name": label,
                "body": l.body,
                "time_s": finite("time", l.time_s)?,
                "distance_m": finite("distance", l.distance_m)?,
                "descent_rate_m_s": finite("descent rate", l.descent_rate_m_s)?,
            },
        }));
    }
    to_text(&FeatureCollection {
        kind: "FeatureCollection",
        tool: TOOL,
        about,
        features,
    })
}

/// `text` with XML's five special characters escaped.
///
/// # Errors
///
/// [`SimError::Unsupported`] for a control character XML 1.0 forbids (all below U+0020 but tab,
/// line feed and carriage return; U+FFFE and U+FFFF).
fn escape(text: &str) -> Result<String, SimError> {
    let forbidden = |c: char| {
        (c < ' ' && !matches!(c, '\t' | '\n' | '\r')) || matches!(c, '\u{FFFE}' | '\u{FFFF}')
    };
    if text.chars().any(forbidden) {
        return Err(SimError::Unsupported {
            what: "a KML name with a control character XML 1.0 forbids",
        });
    }
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    Ok(out)
}

/// The path and the landings as a KML 2.2 document (OGC 07-147r2) named `name`: a `LineString`
/// of `longitude,latitude,height above mean sea level` with `altitudeMode` `absolute`, and a
/// ground-clamped `Point` per landing of `summary`. An XML comment after the declaration names
/// the program that wrote it, `<!-- FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001 -->`
/// ([`hpr_core::tool::stamp`]).
///
/// # Errors
///
/// [`SimError::Unsupported`] for a track of fewer than two points or a name with a control
/// character XML forbids; [`SimError::Domain`] for a coordinate that isn't finite.
pub fn kml(track: &[TrackPoint], summary: &FlightSummary, name: &str) -> Result<String, SimError> {
    kml_with(track, summary, name, "")
}

/// The path and the landings as [`kml()`] writes them, with `description` as the `Document`'s
/// `description` (OGC 07-147r2 section 9.1.3.6, after its `name`), such as the motor catalog's
/// date and how far to trust the path: a map shows it when the file is opened. An empty
/// `description` writes none, as [`kml()`] does.
///
/// # Errors
///
/// As [`kml()`]; also [`SimError::Unsupported`] for a description with a control character XML
/// forbids.
pub fn kml_with(
    track: &[TrackPoint],
    summary: &FlightSummary,
    name: &str,
    description: &str,
) -> Result<String, SimError> {
    too_short(track)?;
    let mut path = Vec::with_capacity(track.len());
    for p in track {
        path.push(format!(
            "{:?},{:?},{:?}",
            finite("longitude", p.longitude_deg)?,
            finite("latitude", p.latitude_deg)?,
            finite("height", p.height_above_sea_level_m)?,
        ));
    }
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    // The stamp holds no `--` and doesn't end in `-`, which an XML comment can't (XML 1.0 §2.5).
    out.push_str(&format!("<!-- {} -->\n", hpr_core::tool::stamp()));
    out.push_str("<kml xmlns=\"http://www.opengis.net/kml/2.2\">\n<Document>\n");
    out.push_str(&format!("<name>{}</name>\n", escape(name)?));
    if !description.is_empty() {
        out.push_str(&format!(
            "<description>{}</description>\n",
            escape(description)?
        ));
    }
    out.push_str("<Placemark>\n<name>flight path</name>\n<LineString>\n");
    out.push_str("<altitudeMode>absolute</altitudeMode>\n<coordinates>\n");
    out.push_str(&path.join("\n"));
    out.push_str("\n</coordinates>\n</LineString>\n</Placemark>\n");
    for (label, l) in landings(summary) {
        out.push_str(&format!(
            "<Placemark>\n<name>{}</name>\n<description>t = {:?} s</description>\n<Point>\n\
             <coordinates>{:?},{:?}</coordinates>\n</Point>\n</Placemark>\n",
            escape(&label)?,
            finite("time", l.time_s)?,
            finite("longitude", l.longitude_deg)?,
            finite("latitude", l.latitude_deg)?,
        ));
    }
    out.push_str("</Document>\n</kml>\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use hpr_atmos::ConstantWind;

    use super::*;
    use crate::flight::{FlightSettings, Simulation};
    use crate::metrics::FlightMetrics;
    use crate::rail::Rail;
    use crate::recorder::Channel;
    use crate::recovery::{Device, DeviceDrag, Trigger};
    use crate::testing::{design, windy_environment};

    /// Valetudo under a canopy from apogee, carried east by a 6 m/s west wind: its recording at
    /// 1 s, its path and its summary.
    pub(super) fn flown() -> (Recorder, Vec<TrackPoint>, FlightSummary, Environment) {
        let wind = ConstantWind::new(6.0, 1.5 * std::f64::consts::PI).unwrap();
        let canopy = Device::new(
            "main",
            DeviceDrag::DragArea { cd_s_m2: 0.5 },
            Trigger::Apogee,
        );
        let sim = Simulation::new(
            &design("rocketpy-valetudo"),
            "example",
            windy_environment(wind).with_geoid_undulation_m(-25.0),
            Rail::vertical(3.0),
            FlightSettings {
                max_time_s: 600.0,
                ..FlightSettings::default()
            },
        )
        .unwrap()
        .with_recovery(vec![canopy])
        .unwrap();
        let recorder = Recorder::new(
            vec![Channel::Time, Channel::CgPosition, Channel::Mach],
            Some(1.0),
        )
        .unwrap();
        let mut watchers = (FlightMetrics::new(), recorder);
        let result = sim.run(&mut watchers).unwrap();
        let (metrics, recorder) = watchers;
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        let track = track(&recorder, sim.environment()).unwrap();
        (recorder, track, summary, sim.environment().clone())
    }

    #[test]
    fn csv_and_json_hold_every_recorded_value_exactly() {
        let (recorder, ..) = flown();
        assert!(recorder.rows().len() > 20);
        let text = csv(&recorder).unwrap();
        let mut lines = text.split("\r\n");
        let header: Vec<&str> = lines.next().unwrap().split(',').collect();
        let columns: Vec<String> = recorder.columns().iter().map(|c| csv_header(c)).collect();
        assert_eq!(header, columns);
        let rows: Vec<Vec<f64>> = lines
            .filter(|l| !l.is_empty())
            .map(|l| l.split(',').map(|c| c.parse().unwrap()).collect())
            .collect();
        assert_eq!(rows, recorder.rows());
        assert!(text.ends_with("\r\n"));

        let value: Value = serde_json::from_str(&json(&recorder).unwrap()).unwrap();
        let columns: Vec<String> = serde_json::from_value(value["columns"].clone()).unwrap();
        let rows: Vec<Vec<f64>> = serde_json::from_value(value["rows"].clone()).unwrap();
        assert_eq!(columns, recorder.columns());
        assert_eq!(rows, recorder.rows());
    }

    /// The object every JSON export carries: name, version and designation.
    fn assert_names_the_tool(value: &Value) {
        assert_eq!(
            value["tool"],
            json!({
                "name": "FusionSpace HPR",
                "version": env!("CARGO_PKG_VERSION"),
                "designation": "FS-ACHERNAR · SW · TOOL 001",
            })
        );
    }

    /// Each export but CSV names the program that wrote it, its version and its designation, as
    /// a title block does: JSON and GeoJSON in a `tool` object written first and second, after
    /// GeoJSON's `type`, and KML in a comment after the declaration. CSV stays bare rows, so a
    /// spreadsheet opens it with the header first.
    #[test]
    fn every_export_but_csv_names_the_program_version_and_designation() {
        let (recorder, track, summary, _) = flown();
        let version = env!("CARGO_PKG_VERSION");
        // The designation's middle dots as escapes: JSON text is ASCII.
        let object = format!(
            "\"tool\":{{\"name\":\"FusionSpace HPR\",\"version\":\"{version}\",\
             \"designation\":\"FS-ACHERNAR \\u00b7 SW \\u00b7 TOOL 001\"}}"
        );

        let text = json(&recorder).unwrap();
        assert!(
            text.starts_with(&format!("{{{object},\"columns\":")),
            "{text:.200}"
        );
        let value: Value = serde_json::from_str(&text).unwrap();
        assert_names_the_tool(&value);
        assert_eq!(value.as_object().unwrap().len(), 3);

        let text = geojson(&track, &summary).unwrap();
        assert!(
            text.starts_with(&format!(
                "{{\"type\":\"FeatureCollection\",{object},\"features\":"
            )),
            "{text:.200}"
        );
        let value: Value = serde_json::from_str(&text).unwrap();
        assert_names_the_tool(&value);
        assert!(geojson_validator().is_valid(&value));

        let text = kml(&track, &summary, "x").unwrap();
        let stamp = format!("<!-- FusionSpace HPR {version} · FS-ACHERNAR · SW · TOOL 001 -->");
        assert_eq!(stamp, format!("<!-- {} -->", hpr_core::tool::stamp()));
        assert!(
            text.starts_with(&format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{stamp}\n<kml "
            )),
            "{text:.200}"
        );
        let document = roxmltree::Document::parse(&text).unwrap();
        let comments: Vec<&str> = document
            .root()
            .children()
            .filter(roxmltree::Node::is_comment)
            .filter_map(|n| n.text())
            .collect();
        assert_eq!(comments, [format!(" {} ", hpr_core::tool::stamp())]);

        let text = csv(&recorder).unwrap();
        assert!(text.starts_with("time [s],"), "{text:.200}");
        assert!(
            !text.contains("FusionSpace") && !text.contains("hpr-sim") && !text.contains("TOOL")
        );
    }

    /// Every column a recorder can keep, as a CSV header writes it: words, then the unit in
    /// brackets in ASCII, as the product system's `data.md` asks (`time [s]`), and the words alone
    /// for a column with no unit. Written out by hand, so a unit read wrongly from a name (`m_s` as
    /// seconds, `m_s2` as meters per second) fails.
    #[test]
    fn a_csv_header_gives_each_columns_unit_in_brackets() {
        let headers: Vec<String> = Channel::ALL
            .iter()
            .flat_map(|c| c.columns())
            .map(|c| csv_header(&c))
            .collect();
        assert_eq!(
            headers.join(","),
            "time [s],position east [m],position north [m],position up [m],\
             velocity east [m/s],velocity north [m/s],velocity up [m/s],\
             attitude w,attitude x,attitude y,attitude z,\
             body rate x [rad/s],body rate y [rad/s],body rate z [rad/s],\
             cg east [m],cg north [m],cg up [m],height above ground [m],vertical speed [m/s],\
             acceleration east [m/s^2],acceleration north [m/s^2],acceleration up [m/s^2],\
             airspeed [m/s],mach,angle of attack [rad],dynamic pressure [Pa],axial coefficient,\
             thrust [N],mass [kg],recovery drag area [m^2]"
        );
        assert!(headers.iter().all(|h| h.is_ascii()));
        // A unit alone is a name, not a unit with no name.
        assert_eq!(csv_header("s"), "s");
        assert_eq!(csv_header("m_s"), "m [s]");
        assert_eq!(csv_header("latitude_deg"), "latitude [deg]");
        assert_eq!(
            csv_header("min_static_margin_cal"),
            "min static margin [cal]"
        );
    }

    /// JSON text is ASCII: a character outside it becomes its escape, one past U+FFFF a surrogate
    /// pair, and the text reads back to the same string.
    #[test]
    fn json_text_is_ascii_and_reads_back_the_same() {
        let value = json!({"name": "Mjölnir · 🚀", "plain": "a\"b\\c"});
        let text = ascii(&serde_json::to_string(&value).unwrap());
        assert_eq!(
            text,
            r#"{"name":"Mj\u00f6lnir \u00b7 \ud83d\ude80","plain":"a\"b\\c"}"#
        );
        assert!(text.is_ascii());
        assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), value);
    }

    /// `json_with` puts the caller's fields between `tool` and `columns`, and refuses one that
    /// would write a key twice.
    #[test]
    fn json_with_adds_the_callers_fields_after_the_tool() {
        let (recorder, ..) = flown();
        #[derive(serde::Serialize)]
        struct About {
            kind: &'static str,
            design: &'static str,
        }
        let text = json_with(
            &recorder,
            &About {
                kind: "simulated",
                design: "r · 1.ork",
            },
        )
        .unwrap();
        assert!(text.is_ascii());
        let at = |key: &str| text.find(&format!("\"{key}\":")).unwrap();
        assert!(
            at("tool") < at("kind") && at("kind") < at("design") && at("design") < at("columns")
        );
        let value: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(value["design"], "r · 1.ork");
        assert_eq!(value.as_object().unwrap().len(), 5);
        assert!(matches!(
            json_with(&recorder, &1),
            Err(SimError::Unsupported { what }) if what.contains("aren't a JSON object")
        ));
        for key in ["tool", "columns", "rows"] {
            let mut twice = serde_json::Map::new();
            twice.insert(key.to_owned(), json!(1));
            assert!(
                matches!(
                    json_with(&recorder, &twice),
                    Err(SimError::Unsupported { what }) if what.contains("tool, columns or rows")
                ),
                "{key}"
            );
        }
    }

    #[test]
    fn a_value_that_is_not_finite_is_refused() {
        let point = TrackPoint {
            time_s: 0.0,
            latitude_deg: 32.0,
            longitude_deg: f64::NAN,
            height_above_ellipsoid_m: 0.0,
            height_above_sea_level_m: 0.0,
        };
        let (.., summary, _) = flown();
        let track = [
            point,
            TrackPoint {
                time_s: 1.0,
                ..point
            },
        ];
        for result in [geojson(&track, &summary), kml(&track, &summary, "x")] {
            assert!(matches!(
                result,
                Err(SimError::Domain { what: "longitude", value }) if value.is_nan()
            ));
        }
        for result in [
            geojson(&track[..1], &summary),
            kml(&track[..1], &summary, "x"),
        ] {
            assert!(matches!(
                result,
                Err(SimError::Unsupported { what }) if what.contains("fewer than two")
            ));
        }
        let fine = [
            TrackPoint {
                longitude_deg: -106.97,
                ..point
            },
            TrackPoint {
                time_s: 1.0,
                longitude_deg: -106.97,
                ..point
            },
        ];
        assert!(matches!(
            kml(&fine, &summary, "a\u{1}b"),
            Err(SimError::Unsupported { what }) if what.contains("control character")
        ));
        assert!(kml(&fine, &summary, "tab\tok").is_ok());

        let recorder = Recorder::with_rows(vec![Channel::Time], vec![vec![0.0], vec![f64::NAN]]);
        for result in [csv(&recorder), json(&recorder)] {
            assert!(matches!(
                result,
                Err(SimError::Domain { what: "recorded value", value }) if value.is_nan()
            ));
        }
    }

    #[test]
    fn track_places_each_row_on_the_ellipsoid_and_the_geoid() {
        let (recorder, track, summary, environment) = flown();
        assert_eq!(track.len(), recorder.rows().len());
        let frame = environment.earth.frame();
        for (point, row) in track.iter().zip(recorder.rows()) {
            let place = frame
                .geodetic_from_enu(DVec3::new(row[1], row[2], row[3]))
                .unwrap();
            assert_eq!(point.time_s, row[0]);
            assert_eq!(point.latitude_deg, place.latitude_rad.to_degrees());
            assert_eq!(point.longitude_deg, place.longitude_rad.to_degrees());
            assert_eq!(point.height_above_ellipsoid_m, place.height_m);
            // h = H + N with N = −25 m: sea-level heights are 25 m above ellipsoidal ones.
            assert!((point.height_above_sea_level_m - place.height_m - 25.0).abs() < 1e-9);
        }
        // The wind carries it east of the site, and it lands where the summary says.
        let last = track.last().unwrap();
        let landing = summary.landing.unwrap();
        assert!(last.longitude_deg > track[0].longitude_deg);
        assert!(landing.east_m > 100.0);

        let without = Recorder::new(vec![Channel::Time], None).unwrap();
        assert!(matches!(
            super::track(&without, &environment),
            Err(SimError::Unsupported { .. })
        ));
    }

    /// The published GeoJSON `FeatureCollection` schema (geojson/schema, MIT, commit 268ba0a).
    fn geojson_validator() -> jsonschema::Validator {
        let schema: Value = serde_json::from_str(include_str!(
            "../tests/data/geojson-feature-collection.schema.json"
        ))
        .unwrap();
        jsonschema::draft7::new(&schema).unwrap()
    }

    #[test]
    fn geojson_passes_the_published_schema_in_longitude_latitude_order() {
        let (_, track, summary, _) = flown();
        let validator = geojson_validator();
        let value: Value = serde_json::from_str(&geojson(&track, &summary).unwrap()).unwrap();
        let errors: Vec<String> = validator
            .iter_errors(&value)
            .map(|e| e.to_string())
            .collect();
        assert!(errors.is_empty(), "{errors:?}");

        let features = value["features"].as_array().unwrap();
        assert_eq!(features.len(), 2);
        let path = features[0]["geometry"]["coordinates"].as_array().unwrap();
        assert_eq!(path.len(), track.len());
        for (position, point) in path.iter().zip(&track) {
            let p: [f64; 3] = serde_json::from_value(position.clone()).unwrap();
            assert_eq!(
                p,
                [
                    point.longitude_deg,
                    point.latitude_deg,
                    point.height_above_ellipsoid_m
                ]
            );
        }
        let landing = summary.landing.unwrap();
        let spot: [f64; 2] =
            serde_json::from_value(features[1]["geometry"]["coordinates"].clone()).unwrap();
        assert_eq!(spot, [landing.longitude_deg, landing.latitude_deg]);

        // The validator does reject a broken document: a one-number position.
        let mut broken = value.clone();
        broken["features"][1]["geometry"]["coordinates"] = json!([landing.longitude_deg]);
        assert!(!validator.is_valid(&broken));
    }

    #[test]
    fn geojson_with_writes_the_fields_between_tool_and_features_and_stays_valid() {
        let (_, track, summary, _) = flown();
        let about = json!({"catalog_as_of": "2026-01-02", "trust": "Rough & <ready>"});
        let text = geojson_with(&track, &summary, &about).unwrap();
        let value: Value = serde_json::from_str(&text).unwrap();
        let errors: Vec<String> = geojson_validator()
            .iter_errors(&value)
            .map(|e| e.to_string())
            .collect();
        assert!(errors.is_empty(), "{errors:?}");
        let at = |key: &str| text.find(&format!("\"{key}\":")).unwrap();
        assert!(
            at("type") < at("tool")
                && at("tool") < at("catalog_as_of")
                && at("catalog_as_of") < at("trust")
                && at("trust") < at("features"),
            "{text:.200}"
        );
        assert_eq!(value["trust"], "Rough & <ready>");
        // Without fields, the bytes are geojson()'s.
        assert_eq!(
            geojson_with(&track, &summary, &serde_json::Map::new()).unwrap(),
            geojson(&track, &summary).unwrap()
        );
        for key in ["type", "tool", "features", "geometry", "properties"] {
            let clash = json!({ key: 1 });
            assert!(
                matches!(
                    geojson_with(&track, &summary, &clash),
                    Err(SimError::Unsupported {
                        what: "a field beside the map named as a GeoJSON member"
                    })
                ),
                "{key}"
            );
        }
        assert!(matches!(
            geojson_with(&track, &summary, &1),
            Err(SimError::Unsupported {
                what: "fields beside the map that aren't a JSON object"
            })
        ));
    }

    #[test]
    fn kml_with_describes_the_document_and_kml_writes_no_description() {
        let (_, track, summary, _) = flown();
        let text = kml_with(&track, &summary, "Flight", "As of 2026-01-02 & <rough>").unwrap();
        let document = roxmltree::Document::parse(&text).unwrap();
        let top = document
            .descendants()
            .find(|n| n.tag_name().name() == "Document")
            .unwrap();
        let children: Vec<&str> = top
            .children()
            .filter(|n| n.is_element())
            .map(|n| n.tag_name().name())
            .take(2)
            .collect();
        assert_eq!(children, ["name", "description"]);
        let description = top
            .children()
            .find(|n| n.tag_name().name() == "description")
            .unwrap();
        assert_eq!(description.text(), Some("As of 2026-01-02 & <rough>"));
        assert_eq!(
            kml_with(&track, &summary, "Flight", "").unwrap(),
            kml(&track, &summary, "Flight").unwrap()
        );
        assert!(
            !kml(&track, &summary, "Flight")
                .unwrap()
                .contains("<Document>\n<name>Flight</name>\n<description>")
        );
    }

    #[test]
    fn kml_parses_as_kml_2_2_with_every_position() {
        let (_, track, summary, _) = flown();
        let text = kml(&track, &summary, "Valetudo & <friends>").unwrap();
        let document = roxmltree::Document::parse(&text).unwrap();
        let root = document.root_element();
        assert_eq!(root.tag_name().name(), "kml");
        assert_eq!(
            root.tag_name().namespace(),
            Some("http://www.opengis.net/kml/2.2")
        );
        let named = |tag: &'static str| {
            let found: Vec<roxmltree::Node<'_, '_>> = document
                .descendants()
                .filter(|n| n.tag_name().name() == tag)
                .collect();
            found.into_iter()
        };
        let name = named("name").next().unwrap().text();
        assert_eq!(name, Some("Valetudo & <friends>"));
        assert_eq!(
            named("altitudeMode").next().unwrap().text(),
            Some("absolute")
        );
        let lists: Vec<&str> = named("coordinates").map(|n| n.text().unwrap()).collect();
        assert_eq!(lists.len(), 2);
        let path: Vec<Vec<f64>> = lists[0]
            .split_whitespace()
            .map(|t| t.split(',').map(|c| c.parse().unwrap()).collect())
            .collect();
        assert_eq!(path.len(), track.len());
        for (position, point) in path.iter().zip(&track) {
            assert_eq!(
                position,
                &[
                    point.longitude_deg,
                    point.latitude_deg,
                    point.height_above_sea_level_m
                ]
            );
        }
        let landing = summary.landing.unwrap();
        let spot: Vec<f64> = lists[1]
            .trim()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        assert_eq!(spot, [landing.longitude_deg, landing.latitude_deg]);
    }
}
