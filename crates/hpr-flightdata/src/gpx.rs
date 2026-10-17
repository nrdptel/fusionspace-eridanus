//! GPX files, the GPS Exchange Format, holding the tracks a GPS receiver, a phone app or a mapping
//! tool recorded, in version 1.0 or 1.1 of the format Topografix publishes.
//!
//! GPX is XML. Its root element `gpx` states its `version` and the program that wrote it
//! (`creator`), then holds waypoints (`wpt`), routes (`rte`, each a list of `rtept`) and tracks
//! (`trk`). A track is a list of segments (`trkseg`), each a list of track points (`trkpt`):
//!
//! ```text
//! <gpx version="1.1" creator="…" xmlns="http://www.topografix.com/GPX/1/1">
//!   <trk><name>Flight 7</name>
//!     <trkseg>
//!       <trkpt lat="35.2000120" lon="-97.1000340">
//!         <ele>402.0</ele><time>2031-07-14T17:04:04.000Z</time><sat>11</sat><hdop>0.8</hdop>
//!       </trkpt>
//!     </trkseg>
//!   </trk>
//! </gpx>
//! ```
//!
//! **What the sources set.** Topografix's schemas, *GPX 1.0* (topografix.com/GPX/1/0/gpx.xsd)
//! and *GPX 1.1* (topografix.com/GPX/1/1/gpx.xsd), and GPX 1.0's manual
//! (topografix.com/gpx_manual.asp). GPX 1.0's schema holds types and no documentation, so the
//! words quoted here are GPX 1.1's schema's; GPX 1.0's manual says the same of each element it
//! describes. A point's `lat` and `lon` are in "Decimal degrees, WGS84 datum", the latitude from
//! −90 to 90 and the longitude from −180 to 180 (both schemas' types: 1.1 leaves out 180 itself,
//! 1.0 doesn't; this reader takes 180, the same meridian as −180). Its `ele` is the "Elevation
//! (in meters) of the point", with no datum of its own. GPX 1.1's schema opens with "all
//! coordinates are relative to the WGS84 datum", which could be read as heights above the WGS84
//! ellipsoid, but it doesn't say so of `ele`, nor does the manual, and neither says whether a
//! writer's elevation is above that ellipsoid, above mean sea level or from a barometer. Its
//! `time` is an XML Schema `dateTime`, and "Date and time in are in Univeral \[sic\] Coordinated
//! Time (UTC), not local time!" `sat` is the "Number of satellites used to calculate the GPX fix",
//! `hdop`, `vdop` and `pdop` its dilutions of precision, and a `fix` of "none means GPS had no
//! fix". In GPX 1.0 a track point may also hold `course`, in "degrees, true", and `speed`, in
//! "meters per second" (the manual); GPX 1.1 dropped both. A track segment "holds a list of Track
//! Points which are logically connected in order. To represent a single GPS track where GPS
//! reception was lost, or the GPS receiver was turned off, start a new Track Segment for each
//! continuous span of track data."
//!
//! **Tracks and channels.** A GPX file holds tracks of positions, not an altimeter's record over
//! time, so the record's primary track is empty and every value is a channel, with a note, as the
//! GPS logs' decision, [ADR-254][adr-254] §2, has it. Each track segment is a GPS track, its
//! channels named after its track's `name`, or `track` if it has none; the segments of a track
//! with several are numbered, `<name> segment 2`, by their place in the track, and a name already
//! used gains ` (2)`, ` (3)` …, so no two segments share a name:
//!
//! | Channel | Quantity | From |
//! |---|---|---|
//! | `<name> latitude`, `<name> longitude` | [`Quantity::Latitude`], [`Quantity::Longitude`] | `lat`, `lon` |
//! | `<name> elevation` | [`Quantity::Altitude`], m | `ele`: an altitude whose datum the file doesn't state |
//! | `<name> satellites` | [`Quantity::Number`] | `sat` |
//! | `<name> HDOP`, `<name> VDOP`, `<name> PDOP` | [`Quantity::Number`] | `hdop`, `vdop`, `pdop` |
//! | `<name> speed` | [`Quantity::Speed`], m/s | GPX 1.0's `speed` |
//! | `<name> course` | [`Quantity::Angle`], ° from true north | GPX 1.0's `course` |
//!
//! A channel is there only if some point of its segment holds its element; a point that lacks it,
//! or holds it empty, such as `<ele/>`, is a gap, `NaN`, the empty ones counted in a note. The
//! rest of a point (`magvar`, `geoidheight`, `fix`, its names and descriptions, `extensions` and
//! other namespaces' elements) is counted in a note and not read.
//! The elevation is [`Quantity::Altitude`], not [`Quantity::GnssAltitude`], whose altitude is
//! above mean sea level as the receiver states it: a GPX file states neither the datum nor the
//! sensor, so only the rise from one point to another means what it says.
//!
//! **Routes and waypoints** are places marked or planned, not flown, so they are kept apart from
//! the tracks, as figures the file states ([`Stated::figures`]), each in the file's order: a
//! waypoint's `waypoint <name> latitude`, `… longitude`, `… elevation` and `… time`, and a route
//! point's `route <name> point <n> latitude` and so on, `<name>` the waypoint's or the route's
//! `name`, or its place among the file's waypoints or routes, `waypoint 2`, if it has none. A time
//! is in seconds from the log's zero, [`Quantity::Duration`]. No reading takes them for the flight.
//!
//! **Times.** A point's time is its `time`, in UTC: one that states an offset from UTC, such as
//! `-04:00`, is moved to UTC; one that states no zone is read as UTC, as GPX 1.1's schema and
//! GPX 1.0's manual say every time is, with a note counting them. The log's zero is the first
//! time the file gives a track point, or, with none, a waypoint or route point, and a note names
//! it. A track point with no time keeps its place with its time a gap, `NaN`. Times within a
//! track go forward, segment to segment: one that goes back is refused by name, as such a track
//! isn't one recording. Two tracks may run in any order, as a file holding two flights does. A
//! second written 60, a leap second's, reads as the next minute's first, as in the KML reader;
//! no sample holds one.
//!
//! **Refused by name.** A document type declaration, so that no entity can expand, and a reference
//! to an entity XML doesn't define; a root that isn't `gpx`, or whose namespace or `version` isn't
//! GPX 1.0's or 1.1's, or whose namespace and `version` disagree; a point with no `lat` or `lon`,
//! or one off the globe (its latitude past ±90° or longitude past ±180°, once read to the nearest
//! double); a value that isn't the schema's kind of number (`ele` and the rest are XML Schema
//! `decimal`s, written with no exponent; `sat` is a whole number); a `course` outside 0° to
//! 360°; a point holding one of the elements read twice; a `time` that isn't a date and time;
//! and a time that goes back within a track. Each names what it found and its line: a
//! point's, or the declaration's, the root's, or the line where the nesting passes the bound
//! (a file of too many nodes is refused at line 1, as the parser gives that refusal no place).
//!
//! **Hostile input.** `roxmltree` parses the XML, with its default refusal of a document type
//! declaration and a limit of [`MAX_NODES`] nodes. The parser descends the tree on the stack, so
//! the text's nesting is counted first, as the KML reader's is ([`crate::kml`]), and a file
//! nesting elements deeper than [`MAX_DEPTH`] is refused. At most [`MAX_TRACKS`] segments,
//! [`MAX_POINTS`] track points and [`MAX_PLACES`] waypoints and route points are read, the rest
//! counted in a note; names are cut to the length a note quotes, and the note listing what isn't
//! read names at most [`MAX_UNREAD_NAMES`] elements.
//!
//! **Where this comes from, and how far to trust it.** The schemas and the manual as cited; no GPX
//! reader was ported. No sample log in the project's collection is a GPX file, so the reader is
//! checked on invented files only, every value against gpxpy 1.6.2 (Apache-2.0), run as an outside
//! reader (`validation/oracles/gpx/gpxpy_read.py`). What a real tracker writes in `ele`, and which
//! of these elements it writes at all, no file has shown yet.
//!
//! [adr-254]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0254-gps-tracks-first-among-the-other-importers.md

use std::collections::{BTreeMap, HashMap, HashSet};

use roxmltree::{Document, Node, ParsingOptions};

use crate::error::LogError;
use crate::kml::{self, Clock};
use crate::log::{Channel, Figure, FlightLog, LogFormat, Quantity, Stated};
use crate::table::short;

/// The format's name in errors.
const FORMAT: &str = "GPX";

/// The namespace of GPX 1.0's elements.
const NAMESPACE_10: &str = "http://www.topografix.com/GPX/1/0";

/// The namespace of GPX 1.1's elements.
const NAMESPACE_11: &str = "http://www.topografix.com/GPX/1/1";

/// The most track segments read from one file, each a track; past it, the rest are counted in a
/// note.
pub const MAX_TRACKS: usize = 256;

/// The most track points read from one file, over all its segments; past it, the rest are counted
/// in a note.
pub const MAX_POINTS: usize = 1_000_000;

/// The most waypoints and route points read from one file, together; past it, the rest are
/// counted in a note. Each gives up to four figures.
pub const MAX_PLACES: usize = 10_000;

/// The most XML nodes the parser builds for one file, so a file of many tiny elements can't fill
/// memory. A track point with its elevation, time, satellite count and three dilutions of
/// precision is about 21 nodes, its whitespace included, so this holds some 190,000 such points,
/// more than two days of a receiver writing once a second.
pub const MAX_NODES: u32 = 4_000_000;

/// The deepest the XML may nest its elements, as the KML reader's bound ([`kml::MAX_DEPTH`]):
/// `roxmltree` descends the tree on the stack, so the nesting is counted before the text is
/// parsed. A track point's `extensions` sit 5 deep (`gpx`, `trk`, `trkseg`, `trkpt`,
/// `extensions`), an app's own elements a few more.
pub const MAX_DEPTH: usize = kml::MAX_DEPTH;

/// The most names of elements not read that the note listing them names; the rest are counted.
pub const MAX_UNREAD_NAMES: usize = 16;

/// Nanoseconds in a second.
const NS_PER_S: i64 = 1_000_000_000;

/// Whether the text is a GPX document, judged from its content, never the file's name: its root
/// element is `gpx`, after an optional byte-order mark, XML declaration, comments and document
/// type declaration. A file with a document type declaration is recognized so that [`read`]
/// refuses it by name.
pub(crate) fn recognizes(text: &str) -> bool {
    kml::root_is(text, "gpx")
}

/// Reads a GPX file's tracks into a flight record, its waypoints and routes as figures it states.
///
/// # Errors
///
/// [`LogError::Syntax`] if the XML isn't well formed, declares a document type, refers to an entity
/// XML doesn't define, nests elements deeper than [`MAX_DEPTH`] or holds more than [`MAX_NODES`]
/// nodes; if its `version` is missing, isn't 1.0 or 1.1, or disagrees with its namespace; or if a
/// point is refused (the module's documentation lists why), naming the element and its line.
/// [`LogError::NotThisFormat`] if its root element isn't `gpx`, or is in a namespace that is
/// neither GPX 1.0's nor 1.1's. [`LogError::NoData`] if it holds no track point.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    read_within(text, MAX_NODES, MAX_POINTS)
}

/// [`read`], parsing at most `max_nodes` XML nodes and reading at most `max_points` track points.
fn read_within(text: &str, max_nodes: u32, max_points: usize) -> Result<FlightLog, LogError> {
    let document = parse(text, max_nodes)?;
    let root = document.root_element();
    let version = version(&document, root)?;
    let mut reader = Reader::new(&document, root.tag_name().namespace(), version);
    reader.max_points = max_points;
    for node in root.children() {
        match reader.own_name(node) {
            Some("wpt") => reader.waypoint(node)?,
            Some("rte") => reader.route(node)?,
            Some("trk") => reader.track(node)?,
            _ => {}
        }
    }
    reader.finish(root)
}

/// A syntax error at a line.
fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// The text parsed as XML, once its nesting is found no deeper than [`MAX_DEPTH`], with at most
/// `nodes` nodes and no document type declaration.
fn parse(text: &str, nodes: u32) -> Result<Document<'_>, LogError> {
    let (deepest, line) = kml::nesting(text);
    if deepest > MAX_DEPTH {
        return Err(syntax(
            line,
            format!(
                "the XML nests elements {deepest} deep, past the {MAX_DEPTH} this reader reads, \
                 so that the parser can't exhaust its stack"
            ),
        ));
    }
    let options = ParsingOptions {
        nodes_limit: nodes,
        ..ParsingOptions::default()
    };
    Document::parse_with_options(text, options).map_err(|error| {
        let mut line = usize::try_from(error.pos().row).unwrap_or(usize::MAX);
        // The parser places this refusal at the text's start; its declaration's line is found.
        if matches!(error, roxmltree::Error::DtdDetected)
            && let Some(offset) = kml::doctype_offset(text)
        {
            line = 1 + text[..offset].matches('\n').count();
        }
        let message = match &error {
            roxmltree::Error::DtdDetected => "the file declares a document type, which this \
                 reader refuses so that no entity can expand"
                .to_owned(),
            roxmltree::Error::UnknownEntityReference(name, _) => format!(
                "the file refers to the entity `&{};`, which XML doesn't define: this reader \
                 expands no entity a file declares",
                short(name)
            ),
            roxmltree::Error::NodesLimitReached => {
                format!("the file holds more than {nodes} XML nodes")
            }
            error => format!("the XML isn't well formed: {}", short(&error.to_string())),
        };
        syntax(line, message)
    })
}

/// The 1-based line a node starts on.
fn line_of(document: &Document<'_>, node: Node<'_, '_>) -> usize {
    usize::try_from(document.text_pos_at(node.range().start).row).unwrap_or(usize::MAX)
}

/// A GPX version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Version {
    V10,
    V11,
}

impl Version {
    fn name(self) -> &'static str {
        match self {
            Self::V10 => "1.0",
            Self::V11 => "1.1",
        }
    }
}

/// The file's GPX version, from its root's `version`, which its namespace, if it states one, must
/// agree with.
fn version(document: &Document<'_>, root: Node<'_, '_>) -> Result<Version, LogError> {
    if root.tag_name().name() != "gpx" {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "its root element is `{}`, not `gpx`",
                short(root.tag_name().name())
            ),
        });
    }
    let by_namespace = match root.tag_name().namespace() {
        None => None,
        Some(NAMESPACE_10) => Some(Version::V10),
        Some(NAMESPACE_11) => Some(Version::V11),
        Some(other) => {
            return Err(LogError::NotThisFormat {
                format: FORMAT,
                message: format!(
                    "its root element `gpx` is in the namespace `{}`, neither GPX 1.0's nor 1.1's",
                    short(other)
                ),
            });
        }
    };
    let line = line_of(document, root);
    let stated = match root.attribute("version") {
        Some("1.0") => Version::V10,
        Some("1.1") => Version::V11,
        Some(other) => {
            return Err(syntax(
                line,
                format!(
                    "the `gpx` element's `version` is `{}`: this reader reads GPX 1.0 and 1.1",
                    short(other)
                ),
            ));
        }
        None => {
            return Err(syntax(
                line,
                "the `gpx` element states no `version`, which GPX 1.0 and 1.1 both require"
                    .to_owned(),
            ));
        }
    };
    if let Some(by_namespace) = by_namespace
        && by_namespace != stated
    {
        return Err(syntax(
            line,
            format!(
                "the `gpx` element's `version` is {} but its namespace is GPX {}'s",
                stated.name(),
                by_namespace.name()
            ),
        ));
    }
    Ok(stated)
}

/// What kind of point an element is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    TrackPoint,
    Waypoint,
    RoutePoint,
}

impl Kind {
    /// The element's name.
    fn element(self) -> &'static str {
        match self {
            Self::TrackPoint => "trkpt",
            Self::Waypoint => "wpt",
            Self::RoutePoint => "rtept",
        }
    }

    /// What it is, for notes: `count` of them.
    fn counted(self, count: usize) -> String {
        counted(
            count,
            match self {
                Self::TrackPoint => "track point",
                Self::Waypoint => "waypoint",
                Self::RoutePoint => "route point",
            },
        )
    }
}

/// A point's time.
#[derive(Debug, Clone)]
struct Stamp {
    /// Nanoseconds since 1970-01-01T00:00:00 UTC.
    ns: i64,
    /// Whether the file stated its zone.
    zoned: bool,
    /// As written, cut to the length a note quotes.
    written: String,
    /// The byte offset of its element, for a refusal's line.
    at: usize,
}

/// A number and its text as written, cut to the length a note quotes.
#[derive(Debug, Clone)]
struct Written {
    value: f64,
    text: String,
}

/// One point read.
#[derive(Debug, Clone, Default)]
struct Point {
    lat: Option<Written>,
    lon: Option<Written>,
    ele: Option<Written>,
    time: Option<Stamp>,
    sat: Option<f64>,
    hdop: Option<f64>,
    vdop: Option<f64>,
    pdop: Option<f64>,
    speed: Option<f64>,
    course: Option<f64>,
}

/// A channel's value at a point, `None` where the point holds none.
type Pick = fn(&Point) -> Option<f64>;

/// One track segment read.
struct Segment {
    /// Its track's name, or `track`.
    track: String,
    /// Its 1-based place in its track, if the track has several.
    number: Option<usize>,
    points: Vec<Point>,
}

/// One waypoint or route point read.
struct Place {
    /// `waypoint <name>` or `route <name> point <n>`, before the names are made unique.
    label: String,
    point: Point,
}

/// What the reader gathers over the file.
struct Reader<'a, 'input> {
    document: &'a Document<'input>,
    namespace: Option<&'a str>,
    version: Version,
    segments: Vec<Segment>,
    places: Vec<Place>,
    tracks: usize,
    waypoints: usize,
    routes: usize,
    points: usize,
    empty_segments: usize,
    empty_tracks: usize,
    segments_left_out: usize,
    segment_points_left_out: usize,
    points_left_out: usize,
    places_left_out: usize,
    no_fix: usize,
    /// Elements not read, by the kind of point and the element's name, at most
    /// [`MAX_UNREAD_NAMES`] of them; other namespaces' elements under one entry.
    unread: BTreeMap<(Kind, String), usize>,
    /// Elements not read whose names are past those listed.
    unread_left_out: usize,
    /// Elements read that hold no value, by the kind of point and the element's name.
    empty: BTreeMap<(Kind, &'input str), usize>,
    /// The most track points read, [`MAX_POINTS`] but in tests.
    max_points: usize,
}

impl<'a, 'input> Reader<'a, 'input> {
    /// A reader of the document, whose root's namespace is GPX's.
    fn new(document: &'a Document<'input>, namespace: Option<&'a str>, version: Version) -> Self {
        Self {
            document,
            namespace,
            version,
            segments: Vec::new(),
            places: Vec::new(),
            tracks: 0,
            waypoints: 0,
            routes: 0,
            points: 0,
            empty_segments: 0,
            empty_tracks: 0,
            segments_left_out: 0,
            segment_points_left_out: 0,
            points_left_out: 0,
            places_left_out: 0,
            no_fix: 0,
            unread: BTreeMap::new(),
            unread_left_out: 0,
            empty: BTreeMap::new(),
            max_points: MAX_POINTS,
        }
    }

    /// A node's name if it is an element in GPX's namespace, the root's.
    fn own_name(&self, node: Node<'_, 'input>) -> Option<&'input str> {
        (node.is_element() && node.tag_name().namespace() == self.namespace)
            .then(|| node.tag_name().name())
    }

    /// The 1-based line of a byte offset, found only for a refusal: it counts the lines from the
    /// text's start, so finding it for every point would take time growing as the square of the
    /// file's length.
    fn line_at(&self, offset: usize) -> usize {
        usize::try_from(self.document.text_pos_at(offset).row).unwrap_or(usize::MAX)
    }

    /// A child element's text, by its name, in GPX's namespace.
    fn name_of(&self, node: Node<'_, 'input>) -> Option<String> {
        node.children()
            .find(|child| self.own_name(*child) == Some("name"))
            .map(|child| clean(&all_text(child)))
            .filter(|name| !name.is_empty())
    }

    /// Counts an element not read.
    fn not_read(&mut self, kind: Kind, name: String) {
        let key = (kind, name);
        if let Some(count) = self.unread.get_mut(&key) {
            *count += 1;
        } else if self.unread.len() < MAX_UNREAD_NAMES {
            self.unread.insert(key, 1);
        } else {
            self.unread_left_out += 1;
        }
    }

    /// Reads a waypoint.
    fn waypoint(&mut self, node: Node<'_, 'input>) -> Result<(), LogError> {
        self.waypoints += 1;
        if self.places.len() >= MAX_PLACES {
            self.places_left_out += 1;
            return Ok(());
        }
        let point = self.point(node, Kind::Waypoint)?;
        let name = self
            .name_of(node)
            .unwrap_or_else(|| self.waypoints.to_string());
        self.places.push(Place {
            label: format!("waypoint {name}"),
            point,
        });
        Ok(())
    }

    /// Reads a route's points.
    fn route(&mut self, node: Node<'_, 'input>) -> Result<(), LogError> {
        self.routes += 1;
        let name = self
            .name_of(node)
            .unwrap_or_else(|| self.routes.to_string());
        let mut number = 0;
        for child in node.children() {
            if self.own_name(child) != Some("rtept") {
                continue;
            }
            number += 1;
            if self.places.len() >= MAX_PLACES {
                self.places_left_out += 1;
                continue;
            }
            let point = self.point(child, Kind::RoutePoint)?;
            self.places.push(Place {
                label: format!("route {name} point {number}"),
                point,
            });
        }
        Ok(())
    }

    /// Reads a track's segments, holding its times to go forward from one point to the next.
    fn track(&mut self, node: Node<'_, 'input>) -> Result<(), LogError> {
        self.tracks += 1;
        let name = self.name_of(node).unwrap_or_else(|| "track".to_owned());
        let segments: Vec<_> = node
            .children()
            .filter(|child| self.own_name(*child) == Some("trkseg"))
            .collect();
        if segments.is_empty() {
            self.empty_tracks += 1;
        }
        let several = segments.len() > 1;
        let mut last: Option<Stamp> = None;
        for (index, segment) in segments.into_iter().enumerate() {
            let trkpts: Vec<_> = segment
                .children()
                .filter(|child| self.own_name(*child) == Some("trkpt"))
                .collect();
            if trkpts.is_empty() {
                self.empty_segments += 1;
                continue;
            }
            if self.segments.len() >= MAX_TRACKS {
                self.segments_left_out += 1;
                self.segment_points_left_out += trkpts.len();
                continue;
            }
            let mut points = Vec::new();
            for trkpt in trkpts {
                if self.points >= self.max_points {
                    self.points_left_out += 1;
                    continue;
                }
                let point = self.point(trkpt, Kind::TrackPoint)?;
                if let Some(stamp) = &point.time {
                    if let Some(previous) = &last
                        && stamp.ns < previous.ns
                    {
                        return Err(syntax(
                            self.line_at(stamp.at),
                            format!(
                                "a time that goes back: track `{name}`'s point is timed `{}`, \
                                 before `{}`, the time of the point ahead of it at line {}; a \
                                 track's times go forward, so this one isn't read as one \
                                 recording",
                                stamp.written,
                                previous.written,
                                self.line_at(previous.at)
                            ),
                        ));
                    }
                    last = Some(stamp.clone());
                }
                self.points += 1;
                points.push(point);
            }
            if !points.is_empty() {
                self.segments.push(Segment {
                    track: name.clone(),
                    number: several.then_some(index + 1),
                    points,
                });
            }
        }
        Ok(())
    }

    /// Reads a point: its position, then each element it holds.
    fn point(&mut self, node: Node<'_, 'input>, kind: Kind) -> Result<Point, LogError> {
        let at = node.range().start;
        let element = kind.element();
        let mut point = Point::default();
        for (attribute, latitude) in [("lat", true), ("lon", false)] {
            let Some(text) = node.attribute(attribute) else {
                return Err(syntax(
                    self.line_at(at),
                    format!(
                        "a `{element}` has no `{attribute}`, which GPX requires of every point"
                    ),
                ));
            };
            let value = decimal(text)
                .ok_or_else(|| not_a_number(self.line_at(at), element, attribute, text))?;
            let on_globe = if latitude {
                (-90.0..=90.0).contains(&value)
            } else {
                (-180.0..=180.0).contains(&value)
            };
            if !on_globe {
                return Err(syntax(
                    self.line_at(at),
                    format!(
                        "a point off the globe: a `{element}`'s `{attribute}` is {}, where a {} \
                         is {}",
                        short(trimmed(text)),
                        if latitude { "latitude" } else { "longitude" },
                        if latitude {
                            "−90° to 90°"
                        } else {
                            "−180° to 180°"
                        }
                    ),
                ));
            }
            let written = Some(Written {
                value,
                text: short(trimmed(text)),
            });
            if latitude {
                point.lat = written;
            } else {
                point.lon = written;
            }
        }
        let mut seen: Vec<&str> = Vec::new();
        for child in node.children().filter(|child| child.is_element()) {
            let Some(name) = self.own_name(child) else {
                self.not_read(kind, "other namespaces' elements".to_owned());
                continue;
            };
            let read_here = match name {
                "ele" | "time" => true,
                "sat" | "hdop" | "vdop" | "pdop" => kind == Kind::TrackPoint,
                "course" | "speed" => kind == Kind::TrackPoint && self.version == Version::V10,
                _ => false,
            };
            if !read_here {
                if name == "fix" && kind == Kind::TrackPoint && all_text(child).trim() == "none" {
                    self.no_fix += 1;
                }
                // A waypoint's `name` names its figures.
                if !(name == "name" && kind == Kind::Waypoint) {
                    self.not_read(kind, format!("`{}`", short(name)));
                }
                continue;
            }
            let child_at = child.range().start;
            if seen.contains(&name) {
                return Err(syntax(
                    self.line_at(child_at),
                    format!("a `{element}` holds two `{name}` elements, where GPX allows one"),
                ));
            }
            seen.push(name);
            let text = all_text(child);
            // An empty element, such as `<ele/>`, states nothing: a gap, counted.
            if trimmed(&text).is_empty() {
                *self.empty.entry((kind, name)).or_insert(0) += 1;
                continue;
            }
            if name == "time" {
                let read = stamp(&text, child_at);
                point.time = Some(read.ok_or_else(|| {
                    syntax(
                        self.line_at(child_at),
                        format!(
                            "a `{element}`'s `time` reads `{}`, not a date and time this reader \
                             reads, such as `2031-07-14T17:04:04.250Z`",
                            short(trimmed(&text))
                        ),
                    )
                })?);
                continue;
            }
            if name == "sat" {
                let count = whole_number(&text)
                    .ok_or_else(|| not_a_number(self.line_at(child_at), element, name, &text))?;
                point.sat = Some(count);
                continue;
            }
            let value = decimal(&text)
                .ok_or_else(|| not_a_number(self.line_at(child_at), element, name, &text))?;
            match name {
                "ele" => {
                    point.ele = Some(Written {
                        value,
                        text: short(trimmed(&text)),
                    });
                }
                "hdop" => point.hdop = Some(value),
                "vdop" => point.vdop = Some(value),
                "pdop" => point.pdop = Some(value),
                "speed" => point.speed = Some(value),
                _ => {
                    if !(0.0..=360.0).contains(&value) {
                        return Err(syntax(
                            self.line_at(child_at),
                            format!(
                                "a `{element}`'s `course` is {}°, outside the 0° to 360° GPX \
                                 1.0 allows",
                                short(trimmed(&text))
                            ),
                        ));
                    }
                    point.course = Some(value);
                }
            }
        }
        Ok(point)
    }

    /// The record: each segment's channels, the places as figures, the notes.
    fn finish(self, root: Node<'_, '_>) -> Result<FlightLog, LogError> {
        if self.segments.is_empty() {
            return Err(LogError::NoData { format: FORMAT });
        }
        let creator = root.attribute("creator").map(clean).unwrap_or_default();
        let mut notes = vec![
            "the record's altitude track is empty: a GPX file holds tracks of positions, not an \
             altimeter's record over time, so every value is a channel"
                .to_owned(),
            format!(
                "read as GPX {}, its `version`: {}, {} with points, each read as a GPS track, {}",
                self.version.name(),
                counted(self.tracks, "track"),
                counted(self.segments.len(), "segment"),
                counted(self.points, "point")
            ),
        ];
        let track_points = || self.segments.iter().flat_map(|segment| &segment.points);
        let place_points = || self.places.iter().map(|place| &place.point);
        if track_points().any(|point| point.ele.is_some()) {
            notes.push(
                "`ele` read in meters, from GPX 1.1's schema and GPX 1.0's manual, as an altitude \
                 whose datum the file doesn't state: they give the unit and no datum, so it may \
                 be above mean sea level, above the WGS84 ellipsoid or above another zero"
                    .to_owned(),
            );
        }
        let zero = track_points()
            .find_map(|point| point.time.as_ref())
            .map(|stamp| (stamp, "a track point"))
            .or_else(|| {
                place_points()
                    .find_map(|point| point.time.as_ref())
                    .map(|stamp| (stamp, "a waypoint or route point"))
            });
        let zero_ns = zero.map(|(stamp, _)| stamp.ns);
        if let Some((stamp, of)) = zero {
            notes.push(format!(
                "the log's zero is `{}`, the first time the file gives {of}",
                stamp.written
            ));
        }
        let unzoned = track_points()
            .chain(place_points())
            .filter(|point| point.time.as_ref().is_some_and(|stamp| !stamp.zoned))
            .count();
        if unzoned > 0 {
            notes.push(format!(
                "times with no zone read as UTC, as GPX 1.1's schema and GPX 1.0's manual say \
                 every time is: {unzoned} of the file's times state none"
            ));
        }
        let untimed = track_points().filter(|point| point.time.is_none()).count();
        if untimed > 0 {
            notes.push(format!(
                "track points with no `time` keep their places, their times gaps: {untimed} of \
                 {}",
                self.points
            ));
        }
        if self.no_fix > 0 {
            notes.push(format!(
                "track points whose `fix` is `none`, where the receiver had no fix, kept as \
                 written: {}",
                self.no_fix
            ));
        }
        if track_points().any(|point| point.speed.is_some() || point.course.is_some()) {
            notes.push(
                "`speed` read in meters per second and `course` in degrees from true north, from \
                 GPX 1.0's manual"
                    .to_owned(),
            );
        }
        if !self.places.is_empty() {
            notes.push(format!(
                "waypoints and routes kept apart from the tracks, as figures the file states, as \
                 places marked or planned, not flown: {}, {}, {} read",
                counted(self.waypoints, "waypoint"),
                counted(self.routes, "route"),
                counted(self.places.len(), "place")
            ));
        }
        if self.empty_tracks + self.empty_segments > 0 {
            notes.push(format!(
                "tracks and segments with no point, left out: {} with no segment, {} with no \
                 point",
                counted(self.empty_tracks, "track"),
                counted(self.empty_segments, "segment")
            ));
        }
        if !self.empty.is_empty() {
            let listed: Vec<String> = self
                .empty
                .iter()
                .map(|((kind, name), count)| format!("`{name}` in {}", kind.counted(*count)))
                .collect();
            notes.push(format!(
                "elements with no value read as gaps: {}",
                listed.join(", ")
            ));
        }
        if !self.unread.is_empty() {
            let mut listed: Vec<String> = self
                .unread
                .iter()
                .map(|((kind, name), count)| format!("{name} in {}", kind.counted(*count)))
                .collect();
            if self.unread_left_out > 0 {
                listed.push(format!(
                    "and {} more of names past the {MAX_UNREAD_NAMES} listed",
                    self.unread_left_out
                ));
            }
            notes.push(format!(
                "elements no reading uses, not read: {}",
                listed.join(", ")
            ));
        }
        let mut left_out = Vec::new();
        if self.segments_left_out > 0 {
            left_out.push(format!(
                "{} past the {MAX_TRACKS} read, holding {}",
                counted(self.segments_left_out, "segment"),
                counted(self.segment_points_left_out, "track point")
            ));
        }
        if self.points_left_out > 0 {
            left_out.push(format!(
                "{} past the {}",
                counted(self.points_left_out, "track point"),
                self.max_points
            ));
        }
        if self.places_left_out > 0 {
            left_out.push(format!(
                "{} past the {MAX_PLACES}",
                match self.places_left_out {
                    1 => "1 waypoint or route point".to_owned(),
                    n => format!("{n} waypoints and route points"),
                }
            ));
        }
        if !left_out.is_empty() {
            notes.push(format!(
                "more than this reader reads, left out: {}",
                left_out.join(", ")
            ));
        }

        let seconds = |stamp: Option<&Stamp>| match (stamp, zero_ns) {
            // Within the reader's dates, late 1677 to early 2262, two times differ by less than
            // 2⁶⁴ ns, and when they differ by less than 2⁵³ ns, 104 days, the difference converts
            // exactly.
            (Some(stamp), Some(zero)) => {
                (i128::from(stamp.ns) - i128::from(zero)) as f64 / NS_PER_S as f64
            }
            _ => f64::NAN,
        };
        let mut names = Names::default();
        let mut channels = Vec::new();
        for segment in &self.segments {
            let base = match segment.number {
                Some(number) => format!("{} segment {number}", segment.track),
                None => segment.track.clone(),
            };
            let name = names.unique(base);
            let time_s: Vec<f64> = segment
                .points
                .iter()
                .map(|point| seconds(point.time.as_ref()))
                .collect();
            let column = |pick: Pick| -> Option<Vec<f64>> {
                segment
                    .points
                    .iter()
                    .any(|point| pick(point).is_some())
                    .then(|| {
                        segment
                            .points
                            .iter()
                            .map(|point| pick(point).unwrap_or(f64::NAN))
                            .collect()
                    })
            };
            let picks: [(&str, Quantity, Pick); 9] = [
                ("latitude", Quantity::Latitude, |p| {
                    p.lat.as_ref().map(|w| w.value)
                }),
                ("longitude", Quantity::Longitude, |p| {
                    p.lon.as_ref().map(|w| w.value)
                }),
                ("elevation", Quantity::Altitude, |p| {
                    p.ele.as_ref().map(|w| w.value)
                }),
                ("satellites", Quantity::Number, |p| p.sat),
                ("HDOP", Quantity::Number, |p| p.hdop),
                ("VDOP", Quantity::Number, |p| p.vdop),
                ("PDOP", Quantity::Number, |p| p.pdop),
                ("speed", Quantity::Speed, |p| p.speed),
                ("course", Quantity::Angle, |p| p.course),
            ];
            for (suffix, quantity, pick) in picks {
                if let Some(values) = column(pick) {
                    channels.push(Channel {
                        column: format!("{name} {suffix}"),
                        quantity,
                        time_s: time_s.clone(),
                        values,
                    });
                }
            }
        }

        let mut figures = Vec::new();
        let mut names = Names::default();
        for place in &self.places {
            let label = names.unique(place.label.clone());
            let point = &place.point;
            for (suffix, quantity, written) in [
                ("latitude", Quantity::Latitude, &point.lat),
                ("longitude", Quantity::Longitude, &point.lon),
                ("elevation", Quantity::Altitude, &point.ele),
            ] {
                if let Some(written) = written {
                    figures.push(Figure::new(
                        format!("{label} {suffix}"),
                        written.text.clone(),
                        Some(quantity),
                        vec![written.value],
                    ));
                }
            }
            if let Some(stamp) = &point.time {
                figures.push(Figure::new(
                    format!("{label} time"),
                    stamp.written.clone(),
                    Some(Quantity::Duration),
                    vec![seconds(Some(stamp))],
                ));
            }
        }

        Ok(FlightLog {
            format: LogFormat::Gpx,
            logger: if creator.is_empty() {
                "GPX".to_owned()
            } else {
                creator
            },
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated {
                figures,
                ..Stated::default()
            },
            time_s: Vec::new(),
            altitude_m: Vec::new(),
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels,
            events: Vec::new(),
            notes,
        })
    }
}

/// `count` of a thing, for notes: `1 track`, `2 tracks`.
fn counted(count: usize, one: &str) -> String {
    if count == 1 {
        format!("1 {one}")
    } else {
        format!("{count} {one}s")
    }
}

/// The names given so far, and for each name given twice the next number to try, so n repeats
/// of one name cost n steps, not n².
#[derive(Default)]
struct Names {
    used: HashSet<String>,
    next: HashMap<String, usize>,
}

impl Names {
    /// A name not yet used, gaining ` (2)`, ` (3)` … if it is; marks it used. A number below the
    /// name's next was taken when it was passed, and a taken name stays taken, so starting there
    /// gives the first free number.
    fn unique(&mut self, name: String) -> String {
        if !self.used.contains(&name) {
            self.used.insert(name.clone());
            return name;
        }
        let number = self.next.entry(name.clone()).or_insert(2);
        loop {
            let candidate = format!("{name} ({number})");
            *number += 1;
            if !self.used.contains(&candidate) {
                self.used.insert(candidate.clone());
                return candidate;
            }
        }
    }
}

/// An element's text, its text and CDATA children joined.
fn all_text(node: Node<'_, '_>) -> String {
    node.children()
        .filter(|child| child.is_text())
        .filter_map(|child| child.text())
        .collect()
}

/// A name as a channel's: its runs of whitespace one space, cut to the length a note quotes.
fn clean(name: &str) -> String {
    short(&name.split_whitespace().collect::<Vec<_>>().join(" "))
}

/// Text with XML's whitespace (space, tab, carriage return, line feed) trimmed from its ends, as
/// XML Schema's numbers and times collapse it.
fn trimmed(text: &str) -> &str {
    text.trim_matches([' ', '\t', '\r', '\n'])
}

/// An XML Schema `decimal`: an optional sign, digits with an optional point, and at least one
/// digit, no exponent; as the nearest `f64`, which must be finite. `None` if it isn't one.
fn decimal(text: &str) -> Option<f64> {
    let text = trimmed(text);
    let unsigned = text.strip_prefix(['+', '-']).unwrap_or(text);
    let (whole, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    let digits = |part: &str| part.bytes().all(|byte| byte.is_ascii_digit());
    if whole.is_empty() && fraction.is_empty() || !digits(whole) || !digits(fraction) {
        return None;
    }
    text.parse::<f64>().ok().filter(|value| value.is_finite())
}

/// An XML Schema `nonNegativeInteger`: an optional `+` and digits, below 2⁵³ so the `f64` is exact.
fn whole_number(text: &str) -> Option<f64> {
    let text = trimmed(text);
    let digits = text.strip_prefix('+').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let value: u64 = digits.parse().ok()?;
    // Exact: below 2⁵³ every integer is an f64.
    (value < 1 << 53).then_some(value as f64)
}

/// A refusal of a value that isn't the schema's kind of number.
fn not_a_number(line: usize, element: &str, name: &str, text: &str) -> LogError {
    let kind = if name == "sat" {
        "a whole number"
    } else {
        "a decimal number, with no exponent"
    };
    syntax(
        line,
        format!(
            "a `{element}`'s `{name}` reads `{}`, not {kind}",
            short(trimmed(text))
        ),
    )
}

/// A `time` as nanoseconds since 1970-01-01T00:00:00 UTC, read by the KML reader's parser of XML
/// Schema's `dateTime` ([`kml::when_ns`]): with `Z` or an offset, moved to UTC; with no zone, as
/// UTC. `None` if it isn't one; `at` is its element's byte offset.
fn stamp(text: &str, at: usize) -> Option<Stamp> {
    let written = trimmed(text);
    let (zoned, ns) = match kml::when_ns(written)? {
        (Clock::Utc, ns) => (true, ns),
        (Clock::Local, ns) => (false, ns),
        _ => return None,
    };
    Some(Stamp {
        ns,
        zoned,
        written: short(written),
        at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A GPX 1.1 file around one track segment's points.
    fn gpx(points: &str) -> String {
        format!(
            "<?xml version=\"1.0\"?>\n<gpx version=\"1.1\" creator=\"test\" \
             xmlns=\"http://www.topografix.com/GPX/1/1\">\n<trk><name>t</name><trkseg>\n{points}\
             \n</trkseg></trk>\n</gpx>\n"
        )
    }

    /// The message of a syntax error, and its line.
    fn refusal(text: &str) -> (usize, String) {
        match read(text) {
            Err(LogError::Syntax { line, message, .. }) => (line, message),
            other => panic!("expected a syntax error, got {other:?}"),
        }
    }

    #[test]
    fn a_document_type_is_refused_by_name_so_no_entity_expands() {
        let text = "<?xml version=\"1.0\"?>\n<!DOCTYPE gpx [<!ENTITY a \"aaaa\">]>\n\
                    <gpx version=\"1.1\"><trk><name>&a;</name></trk></gpx>";
        assert!(recognizes(text));
        let (line, message) = refusal(text);
        assert_eq!(line, 2);
        assert!(
            message.starts_with("the file declares a document type"),
            "{message}"
        );
        // Past a comment naming a document type, the declaration's own line.
        let later = "<?xml version=\"1.0\"?>\n<!-- <!DOCTYPE x> -->\n\n<!DOCTYPE gpx [\n]>\n\
                     <gpx version=\"1.1\"/>";
        assert_eq!(refusal(later).0, 4);
        let external = "<!DOCTYPE gpx SYSTEM \"file:///etc/passwd\"><gpx version=\"1.1\"/>";
        assert!(
            refusal(external)
                .1
                .starts_with("the file declares a document type")
        );
    }

    #[test]
    fn an_undefined_entity_is_refused_by_name() {
        let text = gpx("<trkpt lat=\"1\" lon=\"2\"><name>&rocket;</name></trkpt>");
        let (line, message) = refusal(&text);
        assert_eq!(line, 4);
        assert!(
            message.starts_with("the file refers to the entity `&rocket;`"),
            "{message}"
        );
        // XML's own five entities and character references are text, not entities a file declares.
        let fine = gpx("<trkpt lat=\"1\" lon=\"2\"><name>a &amp; b &#65;</name></trkpt>");
        assert!(read(&fine).is_ok());
    }

    #[test]
    fn a_point_off_the_globe_is_refused_by_name() {
        for (lat, lon, attribute) in [
            ("90.0000001", "0", "lat"),
            ("-91", "0", "lat"),
            ("0", "180.5", "lon"),
            ("0", "-181", "lon"),
        ] {
            let text = gpx(&format!("<trkpt lat=\"{lat}\" lon=\"{lon}\"/>"));
            let (line, message) = refusal(&text);
            assert_eq!(line, 4);
            assert!(
                message.starts_with(&format!(
                    "a point off the globe: a `trkpt`'s `{attribute}` is"
                )),
                "{message}"
            );
        }
        // The poles and the antimeridian are on it.
        for (lat, lon) in [("90", "180"), ("-90", "-180"), ("+0.0", "-0")] {
            let log = read(&gpx(&format!("<trkpt lat=\"{lat}\" lon=\"{lon}\"/>"))).unwrap();
            assert_eq!(log.channels.len(), 2);
        }
        // A waypoint off the globe too.
        let text = "<gpx version=\"1.0\" xmlns=\"http://www.topografix.com/GPX/1/0\">\
                    <wpt lat=\"95\" lon=\"0\"/></gpx>";
        assert!(
            refusal(text)
                .1
                .starts_with("a point off the globe: a `wpt`'s `lat`")
        );
    }

    #[test]
    fn a_time_that_goes_back_within_a_track_is_refused_by_name() {
        let back = gpx(
            "<trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T17:04:05Z</time></trkpt>\n\
             <trkpt lat=\"1\" lon=\"2\"></trkpt>\n\
             <trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T17:04:04.999Z</time></trkpt>",
        );
        let (line, message) = refusal(&back);
        assert_eq!(line, 6);
        assert!(
            message.starts_with(
                "a time that goes back: track `t`'s point is timed `2031-07-14T17:04:04.999Z`, \
                 before `2031-07-14T17:04:05Z`, the time of the point ahead of it at line 4"
            ),
            "{message}"
        );
        // A time written later in another zone can be earlier in UTC: 18:04:04+01:00 is 17:04:04Z.
        let zoned_back = gpx(
            "<trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T17:04:05Z</time></trkpt>\n\
             <trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T18:04:04+01:00</time></trkpt>",
        );
        assert!(refusal(&zoned_back).1.starts_with("a time that goes back"));
        // The same instant in another zone is no step back, and a repeated time is none either.
        let zones = gpx(
            "<trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T17:04:05Z</time></trkpt>\n\
             <trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T13:04:05-04:00</time></trkpt>\n\
             <trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T13:04:05.5-04:00</time></trkpt>",
        );
        let log = read(&zones).unwrap();
        assert_eq!(log.channels[0].time_s, [0.0, 0.0, 0.5]);
        // From one segment to the next of a track, too.
        let segments = "<gpx version=\"1.1\" xmlns=\"http://www.topografix.com/GPX/1/1\"><trk>\
                        <trkseg><trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T17:04:05Z</time>\
                        </trkpt></trkseg><trkseg><trkpt lat=\"1\" lon=\"2\">\
                        <time>2031-07-14T17:04:01Z</time></trkpt></trkseg></trk></gpx>";
        assert!(refusal(segments).1.starts_with("a time that goes back"));
        // But a second track may be earlier than the first.
        let tracks = "<gpx version=\"1.1\" xmlns=\"http://www.topografix.com/GPX/1/1\">\
                      <trk><trkseg><trkpt lat=\"1\" lon=\"2\"><time>2031-07-14T17:04:05Z</time>\
                      </trkpt></trkseg></trk><trk><trkseg><trkpt lat=\"1\" lon=\"2\">\
                      <time>2031-07-14T17:04:01Z</time></trkpt></trkseg></trk></gpx>";
        let log = read(tracks).unwrap();
        assert_eq!(log.channels[2].time_s, [-4.0]);
        assert_eq!(log.channels[2].column, "track (2) latitude");
    }

    #[test]
    fn the_version_and_namespace_must_be_gpx_and_agree() {
        let missing = "<?xml version=\"1.0\"?>\n<gpx xmlns=\"http://www.topografix.com/GPX/1/1\"/>";
        assert_eq!(refusal(missing).0, 2);
        assert!(refusal(missing).1.contains("states no `version`"));
        let unknown = "<gpx version=\"1.2\"/>";
        assert!(refusal(unknown).1.contains("`version` is `1.2`"));
        let disagree = "<gpx version=\"1.0\" xmlns=\"http://www.topografix.com/GPX/1/1\"/>";
        assert!(
            refusal(disagree)
                .1
                .contains("`version` is 1.0 but its namespace is GPX 1.1's")
        );
        let other = "<gpx version=\"1.1\" xmlns=\"http://example.invalid/gpx\"/>";
        assert!(matches!(
            read(other),
            Err(LogError::NotThisFormat { message, .. }) if message.contains("`http://example.invalid/gpx`")
        ));
        assert!(matches!(
            read("<kml/>"),
            Err(LogError::NotThisFormat { message, .. }) if message.contains("`kml`, not `gpx`")
        ));
        // No track point: nothing flown.
        let empty = "<gpx version=\"1.1\"><wpt lat=\"1\" lon=\"2\"/><trk><trkseg/></trk></gpx>";
        assert!(matches!(read(empty), Err(LogError::NoData { .. })));
    }

    #[test]
    fn values_not_the_schemas_numbers_are_refused_by_name() {
        for (inner, message) in [
            (
                "<ele>1e3</ele>",
                "a `trkpt`'s `ele` reads `1e3`, not a decimal number",
            ),
            (
                "<ele>NaN</ele>",
                "a `trkpt`'s `ele` reads `NaN`, not a decimal number",
            ),
            (
                "<ele>.</ele>",
                "a `trkpt`'s `ele` reads `.`, not a decimal number",
            ),
            (
                "<hdop>1,5</hdop>",
                "a `trkpt`'s `hdop` reads `1,5`, not a decimal number",
            ),
            (
                "<sat>7.0</sat>",
                "a `trkpt`'s `sat` reads `7.0`, not a whole number",
            ),
            (
                "<sat>-1</sat>",
                "a `trkpt`'s `sat` reads `-1`, not a whole number",
            ),
            (
                "<ele>1</ele><ele>2</ele>",
                "a `trkpt` holds two `ele` elements",
            ),
            (
                "<time>2031-07-14</time>",
                "a `trkpt`'s `time` reads `2031-07-14`, not a date",
            ),
            (
                "<time>yesterday</time>",
                "a `trkpt`'s `time` reads `yesterday`, not a date",
            ),
        ] {
            let text = gpx(&format!("<trkpt lat=\"1\" lon=\"2\">{inner}</trkpt>"));
            let (line, refused) = refusal(&text);
            assert_eq!(line, 4, "{inner}");
            assert!(refused.starts_with(message), "{inner}: {refused}");
        }
        let text = gpx("<trkpt lat=\"1e1\" lon=\"2\"/>");
        assert!(
            refusal(&text)
                .1
                .starts_with("a `trkpt`'s `lat` reads `1e1`")
        );
        let text = gpx("<trkpt lon=\"2\"/>");
        assert!(refusal(&text).1.starts_with("a `trkpt` has no `lat`"));
        // A course outside 0° to 360°, in GPX 1.0, where a track point holds one.
        let course = "<gpx version=\"1.0\" xmlns=\"http://www.topografix.com/GPX/1/0\"><trk>\
                      <trkseg><trkpt lat=\"1\" lon=\"2\"><course>360.5</course></trkpt>\
                      </trkseg></trk></gpx>";
        assert!(
            refusal(course)
                .1
                .starts_with("a `trkpt`'s `course` is 360.5°")
        );
    }

    #[test]
    fn an_empty_value_is_a_gap_not_a_refusal() {
        let text = gpx(
            "<trkpt lat=\"1\" lon=\"2\"><ele>5</ele><time>2031-07-14T17:04:05Z</time>\
                        </trkpt>\n<trkpt lat=\"1\" lon=\"2\"><ele/><time> </time><sat></sat>\
                        </trkpt>",
        );
        let log = read(&text).unwrap();
        let elevation = &log.channels[2];
        assert_eq!(elevation.column, "t elevation");
        assert_eq!(elevation.values[0], 5.0);
        assert!(elevation.values[1].is_nan() && elevation.time_s[1].is_nan());
        // A satellite count no point states has no channel.
        assert_eq!(log.channels.len(), 3);
        assert!(log.notes.iter().any(|note| note
            == "elements with no value read as gaps: `ele` in 1 track point, `sat` in 1 track \
                point, `time` in 1 track point"));
        // An empty position is no position: refused.
        assert!(
            refusal(&gpx("<trkpt lat=\"\" lon=\"2\"/>"))
                .1
                .starts_with("a `trkpt`'s `lat` reads ``, not a decimal number")
        );
    }

    #[test]
    fn the_xml_is_bounded_before_it_is_read() {
        let deep = format!(
            "<gpx version=\"1.1\">\n{}{}</gpx>",
            "<a>".repeat(MAX_DEPTH),
            "</a>".repeat(MAX_DEPTH)
        );
        let (line, message) = refusal(&deep);
        assert_eq!(line, 2);
        assert!(
            message.contains(&format!("nests elements {} deep", MAX_DEPTH + 1)),
            "{message}"
        );
        let text = gpx(&"<trkpt lat=\"1\" lon=\"2\"/>".repeat(10));
        assert!(read_within(&text, 1_000, MAX_POINTS).is_ok());
        let (line, message) = match read_within(&text, 10, MAX_POINTS) {
            Err(LogError::Syntax { line, message, .. }) => (line, message),
            other => panic!("{other:?}"),
        };
        assert_eq!(message, "the file holds more than 10 XML nodes");
        // The parser places no node count's refusal: it reads the text's first line.
        assert_eq!(line, 1);
    }

    #[test]
    fn past_each_cap_the_rest_is_counted_and_left_out() {
        let note = |log: &FlightLog| {
            log.notes
                .iter()
                .find(|note| note.starts_with("more than this reader reads, left out: "))
                .cloned()
                .unwrap_or_default()
        };
        // Segments: one more than the cap, each of one point.
        let segments: String = (0..=MAX_TRACKS)
            .map(|_| "<trkseg><trkpt lat=\"1\" lon=\"2\"/></trkseg>")
            .collect();
        let text = format!("<gpx version=\"1.1\"><trk>{segments}</trk></gpx>");
        let log = read(&text).unwrap();
        assert_eq!(log.channels.len(), 2 * MAX_TRACKS);
        assert_eq!(
            note(&log),
            format!(
                "more than this reader reads, left out: 1 segment past the {MAX_TRACKS} read, \
                 holding 1 track point"
            )
        );
        // Track points: the cap lowered, as a million would make a slow test.
        let text = gpx(&"<trkpt lat=\"1\" lon=\"2\"/>".repeat(7));
        let log = read_within(&text, MAX_NODES, 5).unwrap();
        assert_eq!(log.channels[0].values.len(), 5);
        assert_eq!(
            note(&log),
            "more than this reader reads, left out: 2 track points past the 5"
        );
        // Waypoints and route points, together.
        let places: String = (0..MAX_PLACES)
            .map(|_| "<wpt lat=\"1\" lon=\"2\"/>")
            .chain(["<rte><rtept lat=\"1\" lon=\"2\"/></rte>"])
            .collect();
        let text = format!(
            "<gpx version=\"1.1\">{places}<trk><trkseg><trkpt lat=\"1\" lon=\"2\"/></trkseg>\
             </trk></gpx>"
        );
        let log = read(&text).unwrap();
        assert_eq!(log.stated.figures.len(), 2 * MAX_PLACES);
        assert_eq!(
            note(&log),
            format!(
                "more than this reader reads, left out: 1 waypoint or route point past the \
                 {MAX_PLACES}"
            )
        );
        // The names of elements not read: those past the cap are counted, not named.
        let names: String = (0..=MAX_UNREAD_NAMES).map(|n| format!("<x{n}/>")).collect();
        let log = read(&gpx(&format!(
            "<trkpt lat=\"1\" lon=\"2\">{names}{names}</trkpt>"
        )))
        .unwrap();
        let unread = log
            .notes
            .iter()
            .find(|note| note.starts_with("elements no reading uses"))
            .unwrap();
        assert_eq!(
            unread.matches(" in 2 track points").count(),
            MAX_UNREAD_NAMES
        );
        assert!(unread.ends_with(&format!(
            "and 2 more of names past the {MAX_UNREAD_NAMES} listed"
        )));
    }

    #[test]
    fn decimals_read_as_xml_schema_writes_them() {
        assert_eq!(decimal(" +1.5\n"), Some(1.5));
        assert_eq!(decimal("-.25"), Some(-0.25));
        assert_eq!(decimal("7."), Some(7.0));
        assert!(decimal("-0").is_some_and(|zero| zero == 0.0 && zero.is_sign_negative()));
        for refused in ["", "+", "-.", "1e3", "inf", "0x10", "1 2", "١"] {
            assert_eq!(decimal(refused), None, "{refused}");
        }
        // A decimal too long for any double is refused, not read as infinity.
        assert_eq!(decimal(&"9".repeat(400)), None);
        assert_eq!(whole_number("+12"), Some(12.0));
        assert_eq!(whole_number(&(1u64 << 53).to_string()), None);
    }

    #[test]
    fn names_are_made_unique_by_number() {
        let mut names = Names::default();
        let given: Vec<String> = ["a", "a", "a (2)", "a", "a (4)", "a", "a"]
            .into_iter()
            .map(|name| names.unique(name.to_owned()))
            .collect();
        assert_eq!(
            given,
            [
                "a",
                "a (2)",
                "a (2) (2)",
                "a (3)",
                "a (4)",
                "a (5)",
                "a (6)"
            ]
        );
        // Each repeat starts from the name's next number, not from 2, so naming the places' cap
        // of one name takes as many steps as there are names.
        assert_eq!(names.next.get("a"), Some(&7));
        let places: String = (0..MAX_PLACES)
            .map(|_| "<wpt lat=\"1\" lon=\"2\"><name>a</name></wpt>")
            .collect();
        let text = format!(
            "<gpx version=\"1.1\">{places}<trk><trkseg><trkpt lat=\"1\" lon=\"2\"/></trkseg>\
             </trk></gpx>"
        );
        let log = read(&text).unwrap();
        let last = &log.stated.figures[log.stated.figures.len() - 1];
        assert_eq!(last.name, format!("waypoint a ({MAX_PLACES}) longitude"));
    }
}
