//! KML files holding a GPS track, as AltOS, Missile Works' RTx, Big Red Bee's BeeLine, fw2kml, GPS
//! Visualizer and others write a flight's path for Google Earth.
//!
//! KML (Keyhole Markup Language) is XML, standardized as OGC 07-147r2 (*OGC KML*, version 2.2.0,
//! 2008). A flight's path is a `Placemark` holding a geometry: a `LineString`, whose
//! `coordinates` list one tuple per point, or Google's extension `gx:Track`, which pairs each
//! point's `when` with its `gx:coord`:
//!
//! ```text
//! <Placemark><name>Flight Track</name>
//!   <LineString><altitudeMode>absolute</altitudeMode>
//!     <coordinates>-97.10000,35.20000,402 -97.10001,35.20002,415</coordinates>
//!   </LineString>
//! </Placemark>
//! <Placemark><name>fw2kml - Flight 0</name>
//!   <gx:Track><altitudeMode>absolute</altitudeMode>
//!     <when>2026-05-23T10:15:00.000Z</when><gx:coord>-97.10000 35.20000 402.0</gx:coord>
//!   </gx:Track>
//! </Placemark>
//! ```
//!
//! **What the standard sets.** A tuple is longitude, latitude and an optional altitude, comma
//! separated, tuples separated by whitespace (§16.9.1); degrees are WGS84's, and "altitude is
//! measured from the vertical datum, which is the WGS84 EGM96 Geoid" (§6.2), in meters
//! (§16.1.1). How a viewer places the altitude is the geometry's `altitudeMode` (§9.17, §16.1.1):
//! `absolute`, "a value in meters relative to the vertical datum"; `relativeToGround`, "in meters
//! relative to the terrain elevation"; `clampToGround`, the default, "ignore the altitude
//! specification". Google adds `gx:altitudeMode`'s `relativeToSeaFloor` and `clampToSeaFloor`,
//! and `gx:Track`, whose "number of `<when>` elements … must be equal to the number of
//! `<gx:coord>` elements", each coordinate "three values for longitude, latitude, and altitude,
//! with no comma separators" (Google, *KML Reference*, developers.google.com/kml/documentation/
//! kmlreference, `<gx:Track>`). A `when` is an XML Schema date-time (§15.3.3.1, §16.10.1), with `Z`
//! for UTC or a `±hh:mm` offset; one that states no zone is "the time in the timezone of some
//! unspecified locality" (W3C, *XML Schema Part 2*, §3.2.7).
//!
//! **Tracks and channels.** A KML file holds tracks of positions, not an altimeter's record over
//! time: even an altitude AltOS took from its barometer (below) is one value of a track's points.
//! So the record's primary track is empty and every value is a channel, with a note, as the GPS
//! logs' decision, [ADR-254][adr-254] §2, has it. Each geometry, in the file's order, inside a
//! `MultiGeometry` too, is a track: each `LineString`, `gx:Track` and `Point`. A track's channels
//! are named after its `Placemark`'s `name`, or `track` if it has none; a placemark holding several
//! geometries numbers them, `<name> part 2`, and a name already used gains ` (2)`, ` (3)` …, so no
//! two tracks share a name:
//!
//! | Channel | Quantity | From |
//! |---|---|---|
//! | `<name> latitude`, `<name> longitude` | [`Quantity::Latitude`], [`Quantity::Longitude`] | the tuple's second and first numbers |
//! | `<name> altitude` | [`Quantity::GnssAltitude`], m | the third number, in mode `absolute` |
//! | `<name> altitude` | [`Quantity::AltitudeMsl`], m | the same, in an AltOS track whose altitude is the barometer's (below) |
//! | `<name> height above ground`, `<name> height above sea floor` | [`Quantity::Distance`], m | the third number, in mode `relativeToGround` or `relativeToSeaFloor`: a height above the terrain, not above sea level |
//! | `<name> altitude` | [`Quantity::Unrecognized`], as written | the third number, in a mode that tells a viewer to ignore it (`clampToGround`, `clampToSeaFloor`, or none stated), in a mode the standard doesn't name, or from GPS Visualizer (below) |
//! | `<name> GPS altitude` | [`Quantity::GnssAltitude`], m | AltOS's comment `alt`, where it differs from the tuple's, in a file where it equals the tuple's at some point |
//! | `<name> GPS altitude` | [`Quantity::Unrecognized`], as written | the same, in a file where it equals the tuple's at no point, so nothing in the file shows its unit |
//! | `<name> Alt` | [`Quantity::Unrecognized`], as written | the `Seconds` layout's `Alt`, where it differs from the tuple's |
//! | `<name> time` | [`Quantity::Unrecognized`], as written | AltOS's comment `time`, where its steps don't show it is in seconds (below; none of the samples) |
//! | `<name> satellites` | [`Quantity::Number`] | a comment's `sats` |
//! | `<name> offset` | [`Quantity::Number`] | the BeeLine comment's first number |
//!
//! A track whose tuples carry no altitude (a two-number tuple's altitude is absent, not 0: the
//! standard calls it optional) has no altitude channel. A `Point` is a track of one sample. A
//! tuple that isn't two or three numbers, or a latitude or longitude out of range, is a gap,
//! `NaN`, counted in a note.
//!
//! **Times.** A point's time is the one the file gives it ([ADR-254][adr-254] §3): a `gx:Track`'s
//! `when`; a `Point` placemark's `TimeStamp`; or, where a writer puts a comment right after a
//! tuple, the time in the comment. The log's zero is the first point the file times, and a note
//! names it. A point the file gives no time keeps its place with its time a gap, `NaN` (§4), as
//! every point of a path without times does, with a note. Times stay in the file's order; a note
//! says how often and how far they go back. A `Point` with a time is also an [`Event`] named after
//! its placemark. Times on two kinds of clock in one file (none of the samples) can't share a zero:
//! those not on the first timed point's clock are gaps, with a note. A time of day that steps back
//! by more than 12 h has passed midnight, and a day is added from there on, at most
//! [`MAX_MIDNIGHTS`] times; the times of day after that are gaps, with a note.
//!
//! **Comments beside points.** Three layouts are read, case aside:
//!
//! - AltOS: `<!-- alt 1000.0000000 time 2.5000000 sats 6 -->`: the GPS receiver's altitude, a
//!   time and its satellite count.
//! - `<!-- Alt 250.0 Seconds 9.5 -->`, in files styled after AltOS's but not named by it.
//! - Big Red Bee's BeeLine: `<!-- 1003 sats:8 UTC 12:34:56 -->`: a number that steps by 15 a
//!   point, the record's place in the logger's memory as far as the files show, the satellite
//!   count, and the UTC time of day.
//!
//! A comment is a point's only if it follows the point's tuple with nothing but whitespace
//! between, and before the next tuple; other comments are counted in a note and not read, nor is
//! their text quoted.
//!
//! **The writers, among the project's sample logs.** As of 2026-10-12, 43 distinct KML files:
//!
//! - *fw2kml* (14), the community converter of Featherweight GPS logs (a Rocketry Forum post,
//!   2023-01-06; github.com/pirate21213/fw2kml): one `Placemark` named `fw2kml - Flight N`. 12 hold
//!   a `LineString` with no times; 2 a `gx:Track` with a `when` per point, in UTC to the tenth of
//!   a second, and a `gx:Tour` of camera views, which isn't a track. One of the two is the same
//!   KML document four times over: a later document identical to the first is skipped with a
//!   note. A later document that differs (none of the samples) is read as more tracks, with a
//!   note, up to [`MAX_DOCUMENTS`].
//! - *Missile Works' RTx* (12): document `RTx/GPS Flight`, KML 2.1's namespace, a byte-order mark
//!   and CR LF; a `Folder` holding `Flight Track`, a path of whole meters with no times, and in 3
//!   files a second path, `Ground Track`, the same positions with no altitude, `clampToGround`.
//!   The RTx's manual says its GPS altitude "is always expressed as Mean Sea Level" (*RTx User
//!   Manual*, 2016-11-26, p13). In 10 of the 12, all from pads near sea level, the altitude reads
//!   exactly 0 for the first hundreds of points, and in none does it go below 0, as a receiver's
//!   altitude clamped at sea level would; it is kept as written.
//! - *AltOS* (11), AltosUI's KML export (Altus Metrum, *AltOS* manual, §12.5.2): a document named
//!   `AO Flight#N S/N: M`, from which the flight number and serial number are read, and a comment
//!   beside every point. 7 files hold one `GPS Altitude` path, whose tuple altitude equals the
//!   comment's `alt`; their `Snippet` names the product and firmware, read into the record. 3
//!   older files hold one path per flight state (`pad`, `boost`, …) and an unset date
//!   (`2147483647-…`, or `0000-00-00`). 1 holds a `Barometric Altitude` path and a `GPS Altitude`
//!   path, each with `Point` placemarks at the flight's events (`Boost (Baro)` …), and a byte that
//!   isn't UTF-8 in its text; AltOS's release notes say that since 1.8.3 the "KML export now
//!   reports both barometric and GPS altitude data". In the flight-state paths the tuple's
//!   altitude climbs while the comment's `alt` holds still with few satellites, and in the
//!   barometric path the two differ at every point: the tuple's is the barometer's. So in an AltOS
//!   track whose tuple altitude differs from its comments' `alt` at more than half of the points
//!   that carry both, the tuple altitude is read as a barometric altitude above sea level,
//!   [`Quantity::AltitudeMsl`], and the comments' `alt` as the GPS altitude, a channel of its own;
//!   otherwise the tuple's is the GPS altitude, and the comments' `alt` a channel only if it
//!   differs somewhere. In every sample track the two differ at all the points or at none, so no
//!   single point decides a track. The comments' `alt` is read in meters where it equals the
//!   tuple's, meters by the standard, at some point of the file, as in the GPS path beside the
//!   barometric one; in the flight-state files it equals it nowhere, so nothing in them shows its
//!   unit and it is kept as written. The `time` is 0 in every comment of the 7 GPS-only files and
//!   the barometric one, which this reader takes as unset: those points have no time. In the
//!   flight-state files it runs on, stepping by about a second between GPS fixes, from an
//!   unstated zero (a negative time on the pad in one, hundreds of seconds in the others). It is
//!   read in seconds where the median step between a track's points of distinct times is 0.5 to
//!   2 s, as a receiver reporting once a second gives and a time in tenths, milliseconds or
//!   minutes wouldn't; otherwise those points have no time, and the `time` is a channel kept as
//!   written. The `Snippet`'s `Max baro height` is read as the stated apogee.
//! - *Big Red Bee's BeeLine* (2): KML 2.0's namespace, CR LF, one `Placemark` named `gps` with a
//!   `MultiGeometry` path of zero-padded whole meters (`00850`), the BeeLine comment beside each
//!   point, in one file comments of the flight's particulars before the first point, and after
//!   the document a comment opening `BRBDATA` holding the logger's memory in hexadecimal, 128
//!   KiB of it: none of these is read, each counted in a note. Big Red Bee's manuals say the
//!   logger records "latitude, longitude, altitude, GPS time, and number of satellites in view"
//!   once a second (*BeeLine GPS*, 2024-11, p3) and writes a KML file for Google Earth (p8); no
//!   manual describes the comments.
//! - *Unnamed, AltOS-styled* (2): styles named as AltOS names them (`ao-flightstate-…`), a
//!   document named by a hexadecimal id, one `Path` in mode `relativeToGround`, and the
//!   `Alt`/`Seconds` comment beside each point; `Alt` equals the tuple's altitude. `Seconds`
//!   runs from a small negative number, a time from liftoff. The logger is `KML`.
//! - *GPS Visualizer* (1), named in the document's `Snippet`: one placemark of 48 paths in a
//!   `MultiGeometry`, 47 of a single point.
//! - *A map drawn in Google Earth* (1): predicted tracks and pins in nested documents, not a
//!   flight log; it is read as any KML file is (the survey of sample logs files it apart, by
//!   id). The logger is `KML`: the file names no writer of its own.
//!
//! **The altitude's unit.** Meters, from the standard, but for GPS Visualizer's. Against a
//! barometric logger flown on the same flight, among the project's sample logs, the track's rise
//! above its first point is 0.99 to 1.06 times the logger's peak in meters in 9 of 11 fw2kml
//! files, 0.73 to 1.15 in 11 of 12 RTx files, 0.76 to 1.06 in 8 of 10 AltOS files and 1.05 in a
//! BeeLine file, where feet would make it 3.28 times. The others are a track whose receiver lost
//! the rocket, two pairings with what may be another flight, and two AltOS files whose first path
//! is the pad's alone. The GPS Visualizer file's rise is 3.24 times its flight's
//! barometric peak in meters: its altitudes are feet written as meters. GPS Visualizer reads a
//! source's altitude "as meters unless "(ft.)" or "(feet)" is included in the field name"
//! (gpsvisualizer.com/tutorials/waypoints.php), so its KML holds meters only when its source
//! said which unit it used, and the file can't say whether it did: its altitude is kept as
//! written, [`Quantity::Unrecognized`]. The two unnamed files' rise is 0.42 times their flight's
//! barometric peak, which neither unit explains; their altitude is read in meters, from the
//! standard, as a height above the ground.
//!
//! Each decision is a note in one grammar, the same before the colon for every file decided
//! alike, such as `` `coordinates` read in meters, from the KML standard: … ``, with this file's
//! evidence after it.
//!
//! **Hostile input.** `roxmltree` parses the XML, with its default refusal of a document type
//! declaration, so no entity can expand, and a limit of [`MAX_NODES`] nodes over all the file's
//! documents together. The parser descends the tree on the stack, so the text's nesting is
//! counted first and a file nesting elements deeper than [`MAX_DEPTH`] is refused. At most
//! [`MAX_TRACKS`] tracks and [`MAX_POINTS`] points are read, the rest counted in a note; notes
//! about single places are capped as every reader's are. Every track's channels hold one value
//! per point read, so the record never holds more than the file wrote.
//!
//! **Where this comes from.** The standard and Google's reference as cited, and the files
//! themselves; no KML reader was ported. Debrief, the project owner's own earlier tool, writes
//! KML but doesn't read it.
//!
//! [adr-254]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0254-gps-tracks-first-among-the-other-importers.md

use std::borrow::Cow;
use std::collections::HashMap;

use roxmltree::{Document, Node, NodeId, ParsingOptions};

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{Notes, number, short};

/// The format's name in errors.
const FORMAT: &str = "KML";

/// The most tracks (geometries) one file gives; past it, the rest are counted in a note. The
/// sample files hold at most 48.
pub const MAX_TRACKS: usize = 256;

/// The most points read from one file, over all its tracks; past it, the rest are counted in a
/// note. The sample files hold at most 8,736.
pub const MAX_POINTS: usize = 1_000_000;

/// The most KML documents read from one file, one after another; past it, the rest are counted in
/// a note. A later document identical to the first doesn't count.
pub const MAX_DOCUMENTS: usize = 8;

/// The most XML nodes the parser builds for one file, over all the documents it holds together, so
/// a file of many tiny elements, in one document or several, can't fill memory. The sample files
/// hold well under 100,000.
pub const MAX_NODES: u32 = 4_000_000;

/// The deepest the XML may nest its elements. The parser underneath, `roxmltree`, descends the tree
/// on the stack, so a file nesting far enough would exhaust it, a crash where an error is due: the
/// nesting is counted before the text is parsed, as the `.ork` reader's is (`hpr_io::ork`). The
/// sample files nest at most 9 deep; 64 is as deep as the `.ork` reader allows.
pub const MAX_DEPTH: usize = 64;

/// The most times a time of day is carried past midnight in one file; the times of day after are
/// gaps. A BeeLine's memory holds a few hours of points at one a second, so a flight's file passes
/// midnight once at most, and the bound keeps the days added from overflowing on a hostile file.
pub const MAX_MIDNIGHTS: i64 = 7;

/// The steps between a track's AltOS `time` values, in seconds, whose median shows the `time` is
/// in seconds: a GPS receiver reporting once a second steps by 1 s, where a time in tenths would
/// step by 10, in milliseconds by 1,000 and in minutes by 1/60. The samples' median is about 1 s.
const ALTOS_STEP_S: std::ops::RangeInclusive<f64> = 0.5..=2.0;

/// The most declarations and comments looked past before the root element when recognizing a
/// file.
const PROLOG_ITEMS: usize = 64;

/// Nanoseconds in a second.
const NS_PER_S: i64 = 1_000_000_000;

/// Nanoseconds in a day.
const DAY_NS: i64 = 86_400 * NS_PER_S;

/// Whether the text is a KML document, judged from its content, never the file's name: its root
/// element is `kml`, after an optional byte-order mark, XML declaration, comments and document
/// type declaration. A file with a document type declaration is recognized so that [`read`]
/// refuses it by name, rather than the importer calling it a layout it doesn't know.
pub(crate) fn recognizes(text: &str) -> bool {
    root_is(text, "kml")
}

/// Whether the text's root element is `name`, unprefixed, after an optional byte-order mark, XML
/// declaration, comments and document type declaration: KML's test, and GPX's
/// ([`crate::gpx`]).
pub(crate) fn root_is(text: &str, name: &str) -> bool {
    let mut rest = text.strip_prefix('\u{feff}').unwrap_or(text);
    for _ in 0..PROLOG_ITEMS {
        rest = rest.trim_start();
        let (open, close) = if rest.starts_with("<?") {
            ("<?", "?>")
        } else if rest.starts_with("<!--") {
            ("<!--", "-->")
        } else if rest.starts_with("<!DOCTYPE") {
            match doctype_length(rest) {
                Some(length) => {
                    rest = &rest[length..];
                    continue;
                }
                None => return false,
            }
        } else {
            break;
        };
        match rest[open.len()..].find(close) {
            Some(end) => rest = &rest[open.len() + end + close.len()..],
            None => return false,
        }
    }
    rest.strip_prefix('<')
        .and_then(|after| after.strip_prefix(name))
        .is_some_and(|after| {
            after.starts_with(|c: char| c.is_ascii_whitespace() || c == '>' || c == '/')
        })
}

/// The length of a document type declaration at the start of the text, `<!DOCTYPE` to its closing
/// `>`, past an internal subset in brackets and quoted text; `None` if it doesn't close.
fn doctype_length(text: &str) -> Option<usize> {
    let mut quote: Option<u8> = None;
    let mut brackets = 0usize;
    for (index, byte) in text.bytes().enumerate() {
        match (quote, byte) {
            (Some(open), _) if byte == open => quote = None,
            (Some(_), _) => {}
            (None, b'"' | b'\'') => quote = Some(byte),
            (None, b'[') => brackets += 1,
            (None, b']') => brackets = brackets.saturating_sub(1),
            (None, b'>') if brackets == 0 => return Some(index + 1),
            (None, _) => {}
        }
    }
    None
}

/// Reads a KML file's tracks into a flight record.
///
/// # Errors
///
/// [`LogError::Syntax`] if the XML isn't well formed, declares a document type, nests elements
/// deeper than [`MAX_DEPTH`], or holds more than [`MAX_NODES`] nodes over its documents;
/// [`LogError::NotThisFormat`] if its root element isn't `kml`; [`LogError::NoData`] if it holds
/// no point.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    read_within(text, MAX_NODES)
}

/// [`read`], with `max_nodes` XML nodes for the whole file, so tests can reach the limit cheaply.
fn read_within(text: &str, max_nodes: u32) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let (pieces, more) = split_documents(text, max_nodes)?;
    let mut copies = 0;
    let mut left_out = usize::from(more);
    let mut first: Option<(&str, Document<'_>)> = None;
    let mut later: Vec<Document<'_>> = Vec::new();
    // The nodes the documents held at once may still take: one budget for the file, not one a
    // document, so that several documents can't hold more than one may.
    let mut nodes_left = max_nodes;
    let mut parse_within = |piece| {
        let document = parse(piece, nodes_left)?;
        let used = u32::try_from(document.descendants().count()).unwrap_or(u32::MAX);
        nodes_left = nodes_left.saturating_sub(used);
        Ok::<_, LogError>(document)
    };
    for piece in pieces {
        if let Some((first_text, _)) = &first {
            if piece.trim() == first_text.trim() {
                copies += 1;
            } else if later.len() + 1 >= MAX_DOCUMENTS {
                left_out += 1;
            } else {
                later.push(parse_within(piece)?);
            }
        } else {
            first = Some((piece, parse_within(piece)?));
        }
    }
    let Some((_, first)) = first else {
        // `split_documents` gives at least one piece, or an error.
        return Err(LogError::NoData { format: FORMAT });
    };
    let mut reader = Reader {
        copies,
        documents_left_out: left_out,
        later_documents: later.len(),
        ..Reader::default()
    };
    for (index, document) in std::iter::once(&first).chain(&later).enumerate() {
        let root = document.root_element();
        if root.tag_name().name() != "kml" {
            return Err(LogError::NotThisFormat {
                format: FORMAT,
                message: format!(
                    "the root element is {:?}, not `kml`",
                    short(root.tag_name().name())
                ),
            });
        }
        reader.read_document(document, index);
    }
    let about = About::of(&first, &reader);
    reader.finish(about)
}

/// The file's text cut into its KML documents, as a writer may write several one after another,
/// each with its own XML declaration; and whether more follow past [`MAX_DOCUMENTS`] × 2 pieces.
/// Each trial parse is dropped before the next, so no more than one is held at a time.
fn split_documents(text: &str, max_nodes: u32) -> Result<(Vec<&str>, bool), LogError> {
    let mut pieces = Vec::new();
    let mut rest = text;
    loop {
        match parse_xml(rest, max_nodes) {
            Ok(_) => {
                pieces.push(rest);
                return Ok((pieces, false));
            }
            Err(XmlFailure::Xml(roxmltree::Error::UnexpectedDeclaration(position))) => {
                let at = offset_of(rest, position)
                    .filter(|&at| at > 0 && rest[at..].starts_with("<?xml"))
                    .ok_or_else(|| {
                        xml_error(&XmlFailure::Xml(roxmltree::Error::UnexpectedDeclaration(
                            position,
                        )))
                    })?;
                pieces.push(&rest[..at]);
                rest = &rest[at..];
                if pieces.len() >= 2 * MAX_DOCUMENTS {
                    return Ok((pieces, true));
                }
            }
            Err(failure) => return Err(xml_error(&failure)),
        }
    }
}

/// Why a text didn't parse.
#[derive(Debug)]
enum XmlFailure {
    /// It nests elements deeper than [`MAX_DEPTH`]: how deep, and the line where it first does.
    Deep { deepest: usize, line: usize },
    /// The parser's own error.
    Xml(roxmltree::Error),
}

/// The text parsed as XML, once its nesting is found no deeper than [`MAX_DEPTH`], with at most
/// `nodes` nodes and no document type declaration.
fn parse_xml(text: &str, nodes: u32) -> Result<Document<'_>, XmlFailure> {
    let (deepest, line) = nesting(text);
    if deepest > MAX_DEPTH {
        return Err(XmlFailure::Deep { deepest, line });
    }
    let options = ParsingOptions {
        nodes_limit: nodes,
        ..ParsingOptions::default()
    };
    Document::parse_with_options(text, options).map_err(XmlFailure::Xml)
}

/// [`parse_xml`], its error the reader's.
fn parse(text: &str, nodes: u32) -> Result<Document<'_>, LogError> {
    parse_xml(text, nodes).map_err(|failure| xml_error(&failure))
}

/// A failure to parse as the reader's error.
fn xml_error(failure: &XmlFailure) -> LogError {
    let (line, message) = match failure {
        XmlFailure::Deep { deepest, line } => (
            *line,
            format!(
                "the XML nests elements {deepest} deep, past the {MAX_DEPTH} this reader reads, \
                 so that the parser can't exhaust its stack"
            ),
        ),
        XmlFailure::Xml(error) => (
            usize::try_from(error.pos().row).unwrap_or(usize::MAX),
            match error {
                roxmltree::Error::DtdDetected => "the file declares a document type, which this \
                     reader refuses so that no entity can expand"
                    .to_owned(),
                roxmltree::Error::NodesLimitReached => {
                    format!("the file holds more than {MAX_NODES} XML nodes over its documents")
                }
                error => format!("the XML isn't well formed: {error}"),
            },
        ),
    };
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// How deep the text nests its elements, counted without parsing it, and the 1-based line where
/// the nesting first passes [`MAX_DEPTH`] (0 if it doesn't). Comments, CDATA sections, processing
/// instructions and declarations change no depth; a `>` inside a quoted attribute value doesn't
/// end its tag. Adapted from the `.ork` reader's count (`hpr_io::ork`), the same project's code;
/// GPX's reader counts with it too ([`crate::gpx`]).
pub(crate) fn nesting(text: &str) -> (usize, usize) {
    let bytes = text.as_bytes();
    let mut index = 0;
    let mut depth = 0usize;
    let mut deepest = 0usize;
    let mut past_at = None;
    while index < bytes.len() {
        if bytes[index] != b'<' {
            index += 1;
            continue;
        }
        let rest = &bytes[index..];
        if let Some(skip) = skipped(rest, b"<!--", b"-->")
            .or_else(|| skipped(rest, b"<![CDATA[", b"]]>"))
            .or_else(|| skipped(rest, b"<?", b"?>"))
        {
            index += skip;
            continue;
        }
        let closing = rest.starts_with(b"</");
        let declaration = rest.starts_with(b"<!");
        let mut cursor = index + 1;
        let mut quote: Option<u8> = None;
        let mut last = 0u8;
        while cursor < bytes.len() {
            let byte = bytes[cursor];
            match quote {
                Some(open) if byte == open => quote = None,
                Some(_) => {}
                None if byte == b'"' || byte == b'\'' => quote = Some(byte),
                None if byte == b'>' => break,
                None => {}
            }
            if !byte.is_ascii_whitespace() {
                last = byte;
            }
            cursor += 1;
        }
        if !declaration {
            if closing {
                depth = depth.saturating_sub(1);
            } else {
                depth += 1;
                deepest = deepest.max(depth);
                if depth > MAX_DEPTH && past_at.is_none() {
                    past_at = Some(index);
                }
                if last == b'/' {
                    depth -= 1;
                }
            }
        }
        index = cursor + 1;
    }
    let line = past_at.map_or(0, |at| {
        bytes[..at].iter().filter(|&&byte| byte == b'\n').count() + 1
    });
    (deepest, line)
}

/// When `rest` starts with `opening`, the length up to and including the next `closing`, or to the
/// end of the text when there is none.
fn skipped(rest: &[u8], opening: &[u8], closing: &[u8]) -> Option<usize> {
    if !rest.starts_with(opening) {
        return None;
    }
    let from = opening.len();
    let found = rest[from..]
        .windows(closing.len())
        .position(|window| window == closing)
        .map(|at| from + at + closing.len());
    Some(found.unwrap_or(rest.len()))
}

/// The byte offset of a parser's position (1-based row, by `\n`, and column, in characters).
fn offset_of(text: &str, position: roxmltree::TextPos) -> Option<usize> {
    let mut line_start = 0;
    for _ in 1..position.row {
        line_start += text.get(line_start..)?.find('\n')? + 1;
    }
    let line = text.get(line_start..)?;
    let column = usize::try_from(position.col).ok()?.checked_sub(1)?;
    line.char_indices()
        .map(|(index, _)| index)
        .chain(std::iter::once(line.len()))
        .nth(column)
        .map(|index| line_start + index)
}

/// How a viewer places a track's altitude: its geometry's `altitudeMode` or `gx:altitudeMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// `absolute`: above the vertical datum, sea level.
    Absolute,
    /// `relativeToGround`: above the terrain.
    RelativeToGround,
    /// `relativeToSeaFloor`: above the sea floor, or the terrain on land.
    RelativeToSeaFloor,
    /// A mode in which a viewer ignores the altitude: as written, or `None` where none is stated
    /// and the default, `clampToGround`, holds.
    Ignored(Option<&'static str>),
    /// A mode the standard doesn't name.
    Unnamed,
}

impl Mode {
    /// The geometry's mode, from its child `altitudeMode` in either namespace.
    fn of(geometry: Node<'_, '_>) -> Self {
        let Some(text) = child(geometry, "altitudeMode").map(|node| all_text(node)) else {
            return Self::Ignored(None);
        };
        match text.trim() {
            "absolute" => Self::Absolute,
            "relativeToGround" => Self::RelativeToGround,
            "relativeToSeaFloor" => Self::RelativeToSeaFloor,
            "clampToGround" => Self::Ignored(Some("clampToGround")),
            "clampToSeaFloor" => Self::Ignored(Some("clampToSeaFloor")),
            _ => Self::Unnamed,
        }
    }
}

/// Which clock a point's time is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Clock {
    /// A `when` with its zone: UTC.
    Utc,
    /// A `when` with no zone: a local time in a zone the file doesn't state.
    Local,
    /// AltOS's comment `time`, seconds from an unstated zero.
    AltOs,
    /// The `Seconds` of a comment, from an unstated zero.
    Seconds,
    /// The BeeLine comment's UTC time of day.
    TimeOfDay,
}

impl Clock {
    /// The word the file names the time by, for notes.
    fn word(self) -> &'static str {
        match self {
            Self::Utc | Self::Local => "`when`",
            Self::AltOs => "`time`",
            Self::Seconds => "`Seconds`",
            Self::TimeOfDay => "`UTC`",
        }
    }
}

/// What AltOS's comment `time` is in a file.
#[derive(Debug, Clone, Copy, PartialEq)]
enum AltosTime {
    /// The file holds no AltOS comment.
    Absent,
    /// 0 beside every point: unset.
    Unset,
    /// In seconds, from the median step between a track's points of distinct times, s.
    Seconds(f64),
    /// In a unit its steps don't show: the median step read as seconds, `NaN` if there is none.
    Unsettled(f64),
}

/// How often a time of day was carried past midnight.
#[derive(Debug, Clone, Copy, Default)]
struct Midnights {
    /// The days added.
    days: usize,
    /// The times of day made gaps past [`MAX_MIDNIGHTS`].
    dropped: usize,
}

/// A point's time as the file writes it: nanoseconds on a clock, and the text it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Stamp<'a> {
    clock: Clock,
    ns: i64,
    raw: &'a str,
}

/// The layout of a comment beside a point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    /// `alt X time Y sats N`.
    AltOs,
    /// `Alt X Seconds Y`.
    Seconds,
    /// `OFFSET sats:N UTC HH:MM:SS`.
    BeeLine,
}

/// What a comment beside a point says.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Remark<'a> {
    layout: Layout,
    /// The comment's altitude, `NaN` if it has none.
    alt: f64,
    /// The comment's time, nanoseconds, and its text.
    time: Option<(i64, &'a str)>,
    /// The satellite count, `NaN` if none.
    sats: f64,
    /// The BeeLine's first number, `NaN` if none.
    offset: f64,
}

/// A comment beside a point, read; `None` if it is in none of the three layouts.
fn remark(text: &str) -> Option<Remark<'_>> {
    let mut tokens = [""; 6];
    let mut count = 0;
    for token in text.split_ascii_whitespace() {
        *tokens.get_mut(count)? = token;
        count += 1;
    }
    let tokens = &tokens[..count];
    if let [offset, sats, utc, time] = tokens
        && utc.eq_ignore_ascii_case("UTC")
        && let Some(sats) = sats.strip_prefix("sats:")
    {
        return Some(Remark {
            layout: Layout::BeeLine,
            alt: f64::NAN,
            time: Some((time_of_day_ns(time)?, *time)),
            sats: number(sats, false)?,
            offset: number(offset, false)?,
        });
    }
    match tokens {
        [alt_key, alt, time_key, time, sats_key, sats]
            if alt_key.eq_ignore_ascii_case("alt")
                && time_key.eq_ignore_ascii_case("time")
                && sats_key.eq_ignore_ascii_case("sats") =>
        {
            Some(Remark {
                layout: Layout::AltOs,
                alt: number(alt, false)?,
                time: Some((decimal_ns(time)?, *time)),
                sats: number(sats, false)?,
                offset: f64::NAN,
            })
        }
        [alt_key, alt, seconds_key, seconds]
            if alt_key.eq_ignore_ascii_case("alt")
                && seconds_key.eq_ignore_ascii_case("seconds") =>
        {
            Some(Remark {
                layout: Layout::Seconds,
                alt: number(alt, false)?,
                time: Some((decimal_ns(seconds)?, *seconds)),
                sats: f64::NAN,
                offset: f64::NAN,
            })
        }
        _ => None,
    }
}

/// A decimal number of seconds as nanoseconds, exactly to the ninth decimal (further digits are
/// dropped): an optional sign, digits, an optional point and digits, no exponent. `None` if it
/// isn't one, or is beyond ±9 × 10⁹ s.
fn decimal_ns(text: &str) -> Option<i64> {
    let (negative, digits) = match text.as_bytes().first()? {
        b'-' => (true, &text[1..]),
        b'+' => (false, &text[1..]),
        _ => (false, text),
    };
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    if whole.is_empty() && fraction.is_empty()
        || digits.ends_with('.')
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|b| b.is_ascii_digit())
        || whole.len() > 10
    {
        return None;
    }
    let seconds: i64 = if whole.is_empty() {
        0
    } else {
        whole.parse().ok()?
    };
    let mut nanos: i64 = 0;
    for digit in fraction.bytes().chain(std::iter::repeat(b'0')).take(9) {
        nanos = nanos * 10 + i64::from(digit - b'0');
    }
    let total = seconds.checked_mul(NS_PER_S)?.checked_add(nanos)?;
    Some(if negative { -total } else { total })
}

/// A time of day, `HH:MM:SS` with optional decimals, as nanoseconds after midnight.
fn time_of_day_ns(text: &str) -> Option<i64> {
    let mut parts = text.split(':');
    let (hours, minutes, seconds) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some()
        || hours.len() != 2
        || minutes.len() != 2
        || !hours
            .bytes()
            .chain(minutes.bytes())
            .all(|b| b.is_ascii_digit())
        || seconds.split('.').next().map(str::len) != Some(2)
    {
        return None;
    }
    let hours: i64 = hours.parse().ok()?;
    let minutes: i64 = minutes.parse().ok()?;
    let seconds = decimal_ns(seconds).filter(|ns| (0..61 * NS_PER_S).contains(ns))?;
    (hours < 24 && minutes < 60).then_some((hours * 3_600 + minutes * 60) * NS_PER_S + seconds)
}

/// A `when` as nanoseconds since 1970-01-01T00:00:00 on its clock: `YYYY-MM-DDThh:mm:ss`, optional
/// decimals, then `Z`, `±hh:mm` or nothing ([`Clock::Local`]). `None` for a date without a time
/// of day (XML Schema's `date`, `gYearMonth`, `gYear`), which places no point in a flight, or for
/// anything else. GPX's reader reads its times with it too ([`crate::gpx`]).
pub(crate) fn when_ns(text: &str) -> Option<(Clock, i64)> {
    let text = text.trim();
    let (date, rest) = text.split_once('T')?;
    let mut fields = date.split('-');
    let (year, month, day) = (fields.next()?, fields.next()?, fields.next()?);
    if fields.next().is_some()
        || year.len() != 4
        || month.len() != 2
        || day.len() != 2
        || !year
            .bytes()
            .chain(month.bytes())
            .chain(day.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let (year, month, day): (i64, i64, i64) =
        (year.parse().ok()?, month.parse().ok()?, day.parse().ok()?);
    if !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
        return None;
    }
    let (time, clock, offset_s) = if let Some(time) = rest.strip_suffix('Z') {
        (time, Clock::Utc, 0)
    } else if let Some(at) = rest.rfind(['+', '-']).filter(|&at| at >= 8) {
        let (time, zone) = rest.split_at(at);
        let sign = if zone.starts_with('-') { -1 } else { 1 };
        let (hours, minutes) = zone[1..].split_once(':')?;
        if hours.len() != 2 || minutes.len() != 2 {
            return None;
        }
        let hours: i64 = hours.parse().ok()?;
        let minutes: i64 = minutes.parse().ok()?;
        if hours > 14 || minutes > 59 {
            return None;
        }
        (time, Clock::Utc, sign * (hours * 3_600 + minutes * 60))
    } else {
        (rest, Clock::Local, 0)
    };
    let of_day = time_of_day_ns(time)?;
    let days = days_from_civil(year, month, day);
    let ns = days
        .checked_mul(DAY_NS)?
        .checked_add(of_day)?
        .checked_sub(offset_s.checked_mul(NS_PER_S)?)?;
    Some((clock, ns))
}

/// Days in a month of the Gregorian calendar.
fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Days from 1970-01-01 to a date of the proleptic Gregorian calendar (H. Hinnant,
/// *chrono-Compatible Low-Level Date Algorithms*, `days_from_civil`, public domain).
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let shifted_month = (month + 9) % 12;
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// A coordinate tuple: longitude, latitude and an optional altitude, from `lon,lat[,alt]` (a
/// `coordinates` tuple) or, if `gx`, `lon lat [alt]` separated by whitespace (a `gx:coord`). `None`
/// if it isn't two or three finite numbers.
fn tuple(text: &str, gx: bool) -> Option<(f64, f64, Option<f64>)> {
    let mut parts: [&str; 3] = [""; 3];
    let count = if gx {
        fill(&mut parts, text.split_ascii_whitespace())?
    } else {
        fill(&mut parts, text.split(','))?
    };
    let value = |part: &str| number(part, false);
    match count {
        2 => Some((value(parts[0])?, value(parts[1])?, None)),
        3 => Some((value(parts[0])?, value(parts[1])?, Some(value(parts[2])?))),
        _ => None,
    }
}

/// Puts the parts in the slots, in order; how many there were, or `None` if more than slots.
fn fill<'t>(slots: &mut [&'t str], parts: impl Iterator<Item = &'t str>) -> Option<usize> {
    let mut count = 0;
    for part in parts {
        *slots.get_mut(count)? = part;
        count += 1;
    }
    Some(count)
}

/// The tuples of a `coordinates` text: split at whitespace, with whitespace beside a comma taken
/// as part of the tuple, as some writers write `lon, lat, alt`.
fn tuples(text: &str) -> Vec<Cow<'_, str>> {
    let mut out: Vec<Cow<'_, str>> = Vec::new();
    let mut current: Option<Cow<'_, str>> = None;
    for token in text.split_ascii_whitespace() {
        current = Some(match current.take() {
            Some(previous) if previous.ends_with(',') || token.starts_with(',') => {
                let mut joined = previous.into_owned();
                joined.push_str(token);
                Cow::Owned(joined)
            }
            Some(previous) => {
                out.push(previous);
                Cow::Borrowed(token)
            }
            None => Cow::Borrowed(token),
        });
    }
    out.extend(current);
    out
}

/// An element's first child element of a local name, in any namespace.
fn child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == name)
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

/// One geometry read: a path, a point or a timed track.
struct Track<'a> {
    /// Its placemark's name, or `track`.
    label: String,
    /// Its placemark, by document and node, to number several geometries in one.
    placemark: Option<(usize, NodeId)>,
    /// Whether it is a `Point`.
    point: bool,
    mode: Mode,
    lat: Vec<f64>,
    lon: Vec<f64>,
    /// Each point's altitude, `NaN` where the tuple has none.
    alt: Vec<f64>,
    stamps: Vec<Option<Stamp<'a>>>,
    remarks: Vec<Option<Remark<'a>>>,
}

/// What the reader gathers over the file's documents.
#[derive(Default)]
struct Reader<'a> {
    tracks: Vec<Track<'a>>,
    tracks_left_out: usize,
    points: usize,
    points_left_out: usize,
    copies: usize,
    later_documents: usize,
    documents_left_out: usize,
    /// Geometries that aren't tracks: polygons and models.
    other_geometries: usize,
    /// Geometries with no point.
    empty_geometries: usize,
    /// Comments in a document outside any `coordinates`.
    loose_comments: usize,
    /// Comments in a `coordinates` beside no point.
    stray_comments: usize,
    /// Comments beside a point in no known layout.
    unknown_comments: usize,
    /// Comments outside a document's root element: count, longest, and whether one opens
    /// `BRBDATA`.
    outside_comments: usize,
    longest_outside: usize,
    memory_dump: bool,
    bad_tuples: usize,
    bad_tuple_example: Option<String>,
    out_of_range: usize,
    bad_whens: usize,
    bad_when_example: Option<String>,
    notes: Notes,
}

impl<'a> Reader<'a> {
    /// Reads one document's tracks and counts its comments.
    fn read_document(&mut self, document: &'a Document<'a>, index: usize) {
        for node in document.root().children() {
            if node.is_comment() {
                let text = node.text().unwrap_or_default();
                self.outside_comments += 1;
                self.longest_outside = self.longest_outside.max(text.chars().count());
                self.memory_dump |= text.trim_start().starts_with("BRBDATA");
            }
        }
        for node in document.root_element().descendants() {
            if node.is_comment() {
                if !node
                    .parent()
                    .is_some_and(|parent| parent.tag_name().name() == "coordinates")
                {
                    self.loose_comments += 1;
                }
                continue;
            }
            if !node.is_element() {
                continue;
            }
            match node.tag_name().name() {
                "LineString" | "Point" | "Track" => self.read_geometry(node, index),
                "Polygon" | "Model" => self.other_geometries += 1,
                _ => {}
            }
        }
    }

    /// Reads a geometry into a track.
    fn read_geometry(&mut self, node: Node<'a, 'a>, document: usize) {
        if self.tracks.len() >= MAX_TRACKS {
            self.tracks_left_out += 1;
            return;
        }
        let placemark = node
            .ancestors()
            .skip(1)
            .find(|ancestor| ancestor.is_element() && ancestor.tag_name().name() == "Placemark");
        let label = placemark
            .and_then(|placemark| child(placemark, "name"))
            .map(|name| clean(&all_text(name)))
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| "track".to_owned());
        let point = node.tag_name().name() == "Point";
        let mut track = Track {
            label,
            placemark: placemark.map(|placemark| (document, placemark.id())),
            point,
            mode: Mode::of(node),
            lat: Vec::new(),
            lon: Vec::new(),
            alt: Vec::new(),
            stamps: Vec::new(),
            remarks: Vec::new(),
        };
        if node.tag_name().name() == "Track" {
            self.read_gx_track(node, &mut track);
        } else if let Some(coordinates) = child(node, "coordinates") {
            self.read_coordinates(coordinates, &mut track);
        }
        if point
            && let Some(when) = placemark
                .and_then(|placemark| child(placemark, "TimeStamp"))
                .and_then(|stamp| child(stamp, "when"))
                .and_then(|when| when.text())
        {
            let stamp = self.stamp_of_when(when);
            track.stamps.fill(stamp);
        }
        if track.lat.is_empty() {
            self.empty_geometries += 1;
        } else {
            self.tracks.push(track);
        }
    }

    /// Adds a point to a track, if the file's points aren't yet [`MAX_POINTS`]; whether it did.
    fn push(
        &mut self,
        track: &mut Track<'a>,
        raw: &str,
        read: Option<(f64, f64, Option<f64>)>,
    ) -> bool {
        if self.points >= MAX_POINTS {
            self.points_left_out += 1;
            return false;
        }
        self.points += 1;
        let (lon, lat, alt) = read.unwrap_or_else(|| {
            self.bad_tuples += 1;
            if self.bad_tuple_example.is_none() {
                self.bad_tuple_example = Some(short(raw));
            }
            (f64::NAN, f64::NAN, Some(f64::NAN))
        });
        let (lon, lat) =
            if lon.is_nan() || ((-180.0..=180.0).contains(&lon) && (-90.0..=90.0).contains(&lat)) {
                (lon, lat)
            } else {
                self.out_of_range += 1;
                (f64::NAN, f64::NAN)
            };
        track.lon.push(lon);
        track.lat.push(lat);
        track.alt.push(alt.unwrap_or(f64::NAN));
        track.stamps.push(None);
        track.remarks.push(None);
        true
    }

    /// Reads a `coordinates` element's tuples and the comments beside them.
    fn read_coordinates(&mut self, coordinates: Node<'a, 'a>, track: &mut Track<'a>) {
        // Whether the last tuple read may still take the comment after it.
        let mut open = false;
        for node in coordinates.children() {
            if node.is_text() {
                for raw in tuples(node.text().unwrap_or_default()) {
                    open = self.push(track, &raw, tuple(&raw, false));
                }
            } else if node.is_comment() {
                let text = node.text().unwrap_or_default();
                if !open {
                    self.stray_comments += 1;
                    continue;
                }
                open = false;
                match (remark(text), track.remarks.last_mut()) {
                    (Some(read), Some(slot)) => *slot = Some(read),
                    _ => self.unknown_comments += 1,
                }
            }
        }
    }

    /// Reads a `gx:Track`'s `when` and `gx:coord` pairs.
    fn read_gx_track(&mut self, node: Node<'a, 'a>, track: &mut Track<'a>) {
        let named = |name: &'static str| {
            node.children()
                .filter(move |child| child.is_element() && child.tag_name().name() == name)
        };
        let (whens, coords) = (named("when").count(), named("coord").count());
        if whens != coords {
            self.notes.push(format!(
                "a `gx:Track` of {:?} holds {whens} `when` and {coords} `gx:coord`, where the \
                 standard has as many of each: the first {} of each are paired, the rest left out",
                track.label,
                whens.min(coords)
            ));
        }
        for (when, coord) in named("when").zip(named("coord")) {
            let raw = coord.text().unwrap_or_default();
            if !self.push(track, raw, tuple(raw, true)) {
                break;
            }
            let stamp = self.stamp_of_when(when.text().unwrap_or_default());
            if let Some(slot) = track.stamps.last_mut() {
                *slot = stamp;
            }
        }
    }

    /// A `when` as a point's time, counting one that isn't a time of day.
    fn stamp_of_when(&mut self, raw: &'a str) -> Option<Stamp<'a>> {
        let read = when_ns(raw).map(|(clock, ns)| Stamp {
            clock,
            ns,
            raw: raw.trim(),
        });
        if read.is_none() {
            self.bad_whens += 1;
            if self.bad_when_example.is_none() {
                self.bad_when_example = Some(short(raw.trim()));
            }
        }
        read
    }

    /// The record: each track's channels, the events, the notes.
    fn finish(mut self, about: About) -> Result<FlightLog, LogError> {
        if self.tracks.is_empty() {
            return Err(LogError::NoData { format: FORMAT });
        }
        let mut notes = vec![
            "the record's altitude track is empty: a KML file holds tracks of positions, not an \
             altimeter's record over time, so every value is a channel"
                .to_owned(),
        ];
        notes.extend(about.note.clone());
        self.document_notes(&mut notes);

        let altos_time = self.altos_time();
        if altos_time == AltosTime::Unset {
            notes.push(
                "the comments' `time` is 0 beside every point, as AltOS leaves it unset in \
                 most of its exports: those points have no time"
                    .to_owned(),
            );
        }
        self.stamp_from_remarks(matches!(altos_time, AltosTime::Seconds(_)));
        let midnights = self.pass_midnight();
        let times = self.times(&mut notes, midnights);
        let names = self.names();
        // AltOS's `alt` shows its unit where it equals the tuple's, meters by the standard.
        let altos_alt_meters = agreement(&self.tracks, Layout::AltOs).0 > 0;

        let mut channels = Vec::new();
        let mut events = Vec::new();
        let mut decided = Decided::default();
        for ((track, time_s), name) in self.tracks.iter().zip(&times).zip(&names) {
            if track.point
                && let Some(&time) = time_s.first()
                && time.is_finite()
            {
                events.push(Event {
                    time_s: time,
                    name: track.label.clone(),
                });
            }
            let mut add = |suffix: &str, quantity: Quantity, values: Vec<f64>| {
                channels.push(Channel {
                    column: format!("{name} {suffix}"),
                    quantity,
                    time_s: time_s.clone(),
                    values,
                });
            };
            add("latitude", Quantity::Latitude, track.lat.clone());
            add("longitude", Quantity::Longitude, track.lon.clone());
            let altitudes = track.alt.iter().filter(|alt| !alt.is_nan()).count();
            let remark_alt: Vec<f64> = track
                .remarks
                .iter()
                .map(|remark| remark.map_or(f64::NAN, |remark| remark.alt))
                .collect();
            let differs =
                track.alt.iter().zip(&remark_alt).any(|(tuple, comment)| {
                    !comment.is_nan() && !(tuple - comment).abs().le(&1e-6)
                });
            let layout = track
                .remarks
                .iter()
                .flatten()
                .map(|remark| remark.layout)
                .next();
            // The points carrying both altitudes, and those where the two differ: an AltOS track
            // is the barometer's where they differ at more than half (all or none in the samples).
            let (both, differing) = track
                .alt
                .iter()
                .zip(&remark_alt)
                .filter(|(tuple, comment)| !tuple.is_nan() && !comment.is_nan())
                .fold((0, 0), |(both, differing), (tuple, comment)| {
                    (
                        both + 1,
                        differing + usize::from((tuple - comment).abs() > 1e-6),
                    )
                });
            let barometric = (layout == Some(Layout::AltOs) && differing * 2 > both)
                .then_some((both, differing));
            if altitudes > 0 {
                let (suffix, quantity) = decided.altitude(track, &about, barometric);
                add(suffix, quantity, track.alt.clone());
            }
            if differs {
                let (suffix, quantity) = decided.remark_altitude(layout, altos_alt_meters);
                add(suffix, quantity, remark_alt);
            }
            if matches!(altos_time, AltosTime::Unsettled(_)) && layout == Some(Layout::AltOs) {
                // The `time` as written: nanoseconds back to the decimal the comment wrote.
                let written: Vec<f64> = track
                    .remarks
                    .iter()
                    .map(|remark| {
                        remark
                            .and_then(|remark| remark.time)
                            .map_or(f64::NAN, |(ns, _)| ns as f64 / NS_PER_S as f64)
                    })
                    .collect();
                add("time", Quantity::Unrecognized, written);
            }
            let sats: Vec<f64> = track
                .remarks
                .iter()
                .map(|remark| remark.map_or(f64::NAN, |remark| remark.sats))
                .collect();
            if sats.iter().any(|sats| !sats.is_nan()) {
                add("satellites", Quantity::Number, sats);
            }
            let offsets: Vec<f64> = track
                .remarks
                .iter()
                .map(|remark| remark.map_or(f64::NAN, |remark| remark.offset))
                .collect();
            if offsets.iter().any(|offset| !offset.is_nan()) {
                add("offset", Quantity::Number, offsets);
            }
        }
        notes.extend(decided.notes(&self.tracks, altos_time));
        notes.extend(self.notes.into_vec());

        Ok(FlightLog {
            format: LogFormat::Kml,
            logger: about.logger,
            serial_number: about.serial_number,
            firmware: about.firmware,
            flight_number: about.flight_number,
            stated: Stated {
                apogee_m: about.apogee_m,
                ..Stated::default()
            },
            time_s: Vec::new(),
            altitude_m: Vec::new(),
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels,
            events,
            notes,
        })
    }

    /// Notes on the documents, the comments and the points left out.
    fn document_notes(&self, notes: &mut Vec<String>) {
        if self.copies > 0 {
            notes.push(format!(
                "the file holds its KML document {} times over, one after another, each the same: \
                 it is read once",
                self.copies + 1
            ));
        }
        if self.later_documents > 0 {
            notes.push(format!(
                "the file holds {} more KML documents after the first, different from it: their \
                 tracks are read after the first's",
                self.later_documents
            ));
        }
        if self.documents_left_out > 0 {
            notes.push(format!(
                "the file holds more KML documents than the {MAX_DOCUMENTS} read: the rest are \
                 left out"
            ));
        }
        if self.outside_comments > 0 {
            let whose = if self.memory_dump {
                ", as Big Red Bee's BeeLine writes its memory in hexadecimal there"
            } else {
                ""
            };
            notes.push(format!(
                "the file holds {} comments outside its document, the longest {} characters{whose}: \
                 they are not read",
                self.outside_comments, self.longest_outside
            ));
        }
        if self.loose_comments > 0 {
            notes.push(format!(
                "the document holds {} comments outside any `coordinates`: they are not read",
                self.loose_comments
            ));
        }
        if self.stray_comments > 0 {
            notes.push(format!(
                "{} comments in `coordinates` follow no point's tuple: they are not read",
                self.stray_comments
            ));
        }
        if self.unknown_comments > 0 {
            notes.push(format!(
                "{} comments beside points are in no layout this reader knows: they are not read",
                self.unknown_comments
            ));
        }
        if self.bad_tuples > 0 {
            notes.push(format!(
                "{} coordinate tuples aren't two or three numbers, such as {:?}: their points are \
                 gaps",
                self.bad_tuples,
                self.bad_tuple_example.as_deref().unwrap_or_default()
            ));
        }
        if self.out_of_range > 0 {
            notes.push(format!(
                "{} points have a latitude beyond ±90° or a longitude beyond ±180°: their \
                 positions are gaps",
                self.out_of_range
            ));
        }
        if self.bad_whens > 0 {
            notes.push(format!(
                "{} `when` aren't a date and time of day, such as {:?}: those points have no time",
                self.bad_whens,
                self.bad_when_example.as_deref().unwrap_or_default()
            ));
        }
        if self.other_geometries > 0 {
            notes.push(format!(
                "the file holds {} polygons or models, which aren't tracks: they are not read",
                self.other_geometries
            ));
        }
        if self.empty_geometries > 0 {
            notes.push(format!(
                "{} geometries hold no point: they are left out",
                self.empty_geometries
            ));
        }
        if self.tracks_left_out > 0 {
            notes.push(format!(
                "the file holds {} more geometries than the {MAX_TRACKS} read: they are left out",
                self.tracks_left_out
            ));
        }
        if self.points_left_out > 0 {
            notes.push(format!(
                "the file holds {} more points than the {MAX_POINTS} read: they are left out",
                self.points_left_out
            ));
        }
    }

    /// What AltOS's comment `time` is in this file: absent, unset (0 beside every point), or in
    /// seconds where the median step between a track's points of distinct times is within
    /// [`ALTOS_STEP_S`].
    fn altos_time(&self) -> AltosTime {
        let tracks = || {
            self.tracks.iter().map(|track| {
                track
                    .remarks
                    .iter()
                    .flatten()
                    .filter(|remark| remark.layout == Layout::AltOs)
                    .filter_map(|remark| remark.time.map(|(ns, _)| ns))
            })
        };
        let mut all = tracks().flatten().peekable();
        if all.peek().is_none() {
            return AltosTime::Absent;
        }
        if all.all(|ns| ns == 0) {
            return AltosTime::Unset;
        }
        let mut steps: Vec<i64> = Vec::new();
        for times in tracks() {
            let times: Vec<i64> = times.collect();
            steps.extend(
                times
                    .windows(2)
                    .map(|pair| pair[1].saturating_sub(pair[0]))
                    .filter(|step| *step > 0),
            );
        }
        steps.sort_unstable();
        let median = steps
            .get(steps.len() / 2)
            .map_or(f64::NAN, |step| *step as f64 / NS_PER_S as f64);
        if ALTOS_STEP_S.contains(&median) {
            AltosTime::Seconds(median)
        } else {
            AltosTime::Unsettled(median)
        }
    }

    /// Gives each point without a time the one its comment states, if any; AltOS's only if
    /// `altos_seconds`.
    fn stamp_from_remarks(&mut self, altos_seconds: bool) {
        for track in &mut self.tracks {
            for (stamp, remark) in track.stamps.iter_mut().zip(&track.remarks) {
                let Some(remark) = remark else { continue };
                let clock = match remark.layout {
                    Layout::AltOs if !altos_seconds => continue,
                    Layout::AltOs => Clock::AltOs,
                    Layout::Seconds => Clock::Seconds,
                    Layout::BeeLine => Clock::TimeOfDay,
                };
                if stamp.is_none()
                    && let Some((ns, raw)) = remark.time
                {
                    *stamp = Some(Stamp { clock, ns, raw });
                }
            }
        }
    }

    /// Carries a time of day past midnight: where it steps back by more than 12 h, a day is
    /// added from there on, up to [`MAX_MIDNIGHTS`] days. From a step back past that bound on,
    /// the times of day are gaps, as their day can't be told.
    fn pass_midnight(&mut self) -> Midnights {
        let mut days: i64 = 0;
        let mut previous: Option<i64> = None;
        let mut passed = Midnights::default();
        for stamp in self
            .tracks
            .iter_mut()
            .flat_map(|track| track.stamps.iter_mut())
            .filter(|stamp| stamp.is_some_and(|stamp| stamp.clock == Clock::TimeOfDay))
        {
            if passed.dropped > 0 {
                *stamp = None;
                passed.dropped += 1;
                continue;
            }
            let Some(read) = stamp.as_mut() else { continue };
            // A time of day is under 24 h and 61 s, and `days` at most `MAX_MIDNIGHTS`, so no
            // sum here comes near overflowing.
            let mut ns = read.ns + days * DAY_NS;
            if previous.is_some_and(|previous| ns < previous - DAY_NS / 2) {
                if days >= MAX_MIDNIGHTS {
                    *stamp = None;
                    passed.dropped = 1;
                    continue;
                }
                days += 1;
                ns += DAY_NS;
            }
            read.ns = ns;
            previous = Some(ns);
        }
        passed.days = usize::try_from(days).unwrap_or(0);
        passed
    }

    /// Each track's times, s from the log's zero, and the notes on them.
    fn times(&self, notes: &mut Vec<String>, midnights: Midnights) -> Vec<Vec<f64>> {
        let all = || {
            self.tracks
                .iter()
                .flat_map(|track| track.stamps.iter().flatten())
        };
        let zero = all().next().copied();
        let points: usize = self.tracks.iter().map(|track| track.lat.len()).sum();
        let mut other_clock = 0;
        let times: Vec<Vec<f64>> = self
            .tracks
            .iter()
            .map(|track| {
                track
                    .stamps
                    .iter()
                    .map(|stamp| match (stamp, zero) {
                        (Some(stamp), Some(zero)) if stamp.clock == zero.clock => stamp
                            .ns
                            .checked_sub(zero.ns)
                            .map_or(f64::NAN, |ns| ns as f64 / NS_PER_S as f64),
                        (Some(_), _) => {
                            other_clock += 1;
                            f64::NAN
                        }
                        (None, _) => f64::NAN,
                    })
                    .collect()
            })
            .collect();
        let untimed = times.iter().flatten().filter(|time| time.is_nan()).count();
        match zero {
            None => notes.push(format!(
                "the file gives none of its {points} points a time: their times are gaps"
            )),
            Some(zero) => {
                let zone = match zero.clock {
                    Clock::TimeOfDay => " UTC",
                    _ => "",
                };
                notes.push(format!(
                    "the log's zero is the first point the file times: {} {}{zone}",
                    zero.clock.word(),
                    short(zero.raw)
                ));
                if untimed > 0 {
                    notes.push(format!(
                        "{untimed} of the file's {points} points have no time: their times are \
                         gaps"
                    ));
                }
            }
        }
        if other_clock > 0 {
            notes.push(format!(
                "{other_clock} points are timed on another clock than the first timed point's, \
                 so they can't share its zero: their times are gaps"
            ));
        }
        if midnights.days > 0 {
            notes.push(format!(
                "the time of day steps back past midnight {} times: a day is added at each",
                midnights.days
            ));
        }
        if midnights.dropped > 0 {
            notes.push(format!(
                "the time of day steps back past midnight more than {MAX_MIDNIGHTS} times, more \
                 than a logger's memory holds: the {} times of day from there on are gaps",
                midnights.dropped
            ));
        }
        let mut backs = 0;
        let mut largest: f64 = 0.0;
        for track in &times {
            let mut previous = f64::NAN;
            for &time in track.iter().filter(|time| !time.is_nan()) {
                if time < previous {
                    backs += 1;
                    largest = largest.max(previous - time);
                }
                previous = time;
            }
        }
        if backs > 0 {
            notes.push(format!(
                "the times go back {backs} times within a track, by up to {largest:.3} s: the \
                 points are kept in the file's order"
            ));
        }
        times
    }

    /// Each track's name for its channels: its placemark's, numbered by part where a placemark
    /// holds several geometries, with ` (2)`, ` (3)` … where a name repeats.
    fn names(&self) -> Vec<String> {
        let mut parts: HashMap<(usize, NodeId), usize> = HashMap::new();
        for key in self.tracks.iter().filter_map(|track| track.placemark) {
            *parts.entry(key).or_default() += 1;
        }
        let mut seen_parts: HashMap<(usize, NodeId), usize> = HashMap::new();
        let mut used: HashMap<String, usize> = HashMap::new();
        self.tracks
            .iter()
            .map(|track| {
                let mut name = track.label.clone();
                if let Some(key) = track.placemark
                    && parts.get(&key).copied().unwrap_or(0) > 1
                {
                    let part = seen_parts.entry(key).or_default();
                    *part += 1;
                    name = format!("{name} part {part}");
                }
                let count = used.entry(name.clone()).or_default();
                *count += 1;
                if *count > 1 {
                    name = format!("{name} ({count})");
                }
                name
            })
            .collect()
    }
}

/// What the file says about its writer and logger.
#[derive(Debug, Default)]
struct About {
    logger: String,
    writer: Writer,
    serial_number: Option<String>,
    firmware: Option<String>,
    flight_number: Option<u32>,
    apogee_m: Option<f64>,
    note: Option<String>,
}

/// The program that wrote the file, as far as the file tells.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Writer {
    Fw2kml,
    Rtx,
    AltOs,
    BeeLine,
    GpsVisualizer,
    #[default]
    Unknown,
}

impl About {
    /// The writer and logger, from the first document and the comments read.
    fn of(document: &Document<'_>, reader: &Reader<'_>) -> Self {
        let root = document.root_element();
        let top = child(root, "Document").unwrap_or(root);
        let title = child(top, "name").map(|name| clean(&all_text(name)));
        let snippet = child(top, "Snippet").map(|node| all_text(node));
        let mut about = Self::default();
        if let Some(rest) = title
            .as_deref()
            .and_then(|title| title.strip_prefix("AO Flight#"))
        {
            about.writer = Writer::AltOs;
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            about.flight_number = digits.parse().ok();
            about.serial_number = rest
                .split_once("S/N:")
                .map(|(_, serial)| serial.trim().to_owned())
                .filter(|serial| !serial.is_empty());
            let mut product = None;
            for line in snippet.as_deref().unwrap_or_default().lines() {
                let Some((key, value)) = line.split_once(':') else {
                    continue;
                };
                let value = value.trim();
                match key.trim() {
                    "Product" if !value.is_empty() => product = Some(short(value)),
                    "Firmware" if !value.is_empty() => about.firmware = Some(short(value)),
                    "Max baro height" => about.apogee_m = stated_height_m(value),
                    _ => {}
                }
            }
            about.logger = product.map_or_else(|| "AltOS".to_owned(), |p| format!("AltOS {p}"));
            return about;
        }
        about.writer = if title.as_deref() == Some("RTx/GPS Flight") {
            Writer::Rtx
        } else if root.descendants().any(|node| {
            node.is_element()
                && node.tag_name().name() == "name"
                && node
                    .parent()
                    .is_some_and(|parent| parent.tag_name().name() == "Placemark")
                && all_text(node).trim_start().starts_with("fw2kml")
        }) {
            Writer::Fw2kml
        } else if snippet
            .as_deref()
            .is_some_and(|text| text.contains("GPS Visualizer"))
        {
            Writer::GpsVisualizer
        } else if reader.memory_dump
            || reader
                .tracks
                .iter()
                .flat_map(|track| track.remarks.iter().flatten())
                .any(|remark| remark.layout == Layout::BeeLine)
        {
            Writer::BeeLine
        } else {
            Writer::Unknown
        };
        about.logger = match about.writer {
            Writer::Fw2kml => "fw2kml",
            Writer::Rtx => "Missile Works RTx",
            Writer::BeeLine => "Big Red Bee BeeLine",
            Writer::GpsVisualizer => "GPS Visualizer",
            Writer::AltOs | Writer::Unknown => "KML",
        }
        .to_owned();
        about
    }
}

/// A height AltOS's summary states, such as `5000 ft` or `1500 m`, in meters.
fn stated_height_m(text: &str) -> Option<f64> {
    let (value, unit) = text.trim().split_once(char::is_whitespace)?;
    let value = number(value, false)?;
    match unit.trim() {
        "ft" => Some(value * FOOT_M),
        "m" => Some(value),
        _ => None,
    }
}

/// The unit decisions made over the file's tracks, for their notes.
#[derive(Default)]
struct Decided {
    /// Tracks whose altitude is read in meters from the standard.
    meters: usize,
    /// Of those, AltOS tracks read as the barometer's altitude.
    barometric: usize,
    /// Over those barometric tracks, the points carrying both altitudes and those where the two
    /// differ.
    barometric_both: usize,
    barometric_differing: usize,
    /// Tracks whose altitude GPS Visualizer wrote, kept as written.
    gps_visualizer: usize,
    /// Tracks whose altitude a viewer ignores, by mode.
    ignored: Vec<&'static str>,
    /// Tracks in a mode the standard doesn't name.
    unnamed: usize,
    /// Whether AltOS's comment `alt` became a channel, and if so whether in meters.
    altos_alt: Option<bool>,
    /// Whether the `Seconds` layout's `Alt` became a channel.
    seconds_alt: bool,
}

impl Decided {
    /// A track's altitude channel: its name's suffix and quantity. `barometric` holds, for an
    /// AltOS track whose tuple altitude differs from its comments' `alt` at more than half of the
    /// points carrying both, how many carry both and how many differ.
    fn altitude(
        &mut self,
        track: &Track<'_>,
        about: &About,
        barometric: Option<(usize, usize)>,
    ) -> (&'static str, Quantity) {
        if about.writer == Writer::GpsVisualizer {
            self.gps_visualizer += 1;
            return ("altitude", Quantity::Unrecognized);
        }
        match track.mode {
            Mode::Absolute => {
                self.meters += 1;
                if let Some((both, differing)) = barometric {
                    self.barometric += 1;
                    self.barometric_both += both;
                    self.barometric_differing += differing;
                    ("altitude", Quantity::AltitudeMsl)
                } else {
                    ("altitude", Quantity::GnssAltitude)
                }
            }
            Mode::RelativeToGround => {
                self.meters += 1;
                ("height above ground", Quantity::Distance)
            }
            Mode::RelativeToSeaFloor => {
                self.meters += 1;
                ("height above sea floor", Quantity::Distance)
            }
            Mode::Ignored(mode) => {
                self.ignored
                    .push(mode.unwrap_or("clampToGround, the default, as none is stated"));
                ("altitude", Quantity::Unrecognized)
            }
            Mode::Unnamed => {
                self.unnamed += 1;
                ("altitude", Quantity::Unrecognized)
            }
        }
    }

    /// The channel of a comment's altitude that differs from the tuple's: its name's suffix and
    /// quantity. AltOS's `alt` is the GPS altitude, in meters if `altos_meters` (it equals the
    /// tuple's somewhere in the file), else as written; the `Seconds` layout's `Alt` is kept as
    /// written.
    fn remark_altitude(
        &mut self,
        layout: Option<Layout>,
        altos_meters: bool,
    ) -> (&'static str, Quantity) {
        if layout == Some(Layout::AltOs) {
            self.altos_alt = Some(altos_meters);
            let quantity = if altos_meters {
                Quantity::GnssAltitude
            } else {
                Quantity::Unrecognized
            };
            ("GPS altitude", quantity)
        } else {
            self.seconds_alt = true;
            ("Alt", Quantity::Unrecognized)
        }
    }

    /// One note per decision, with this file's evidence.
    fn notes(&self, tracks: &[Track<'_>], altos_time: AltosTime) -> Vec<String> {
        let mut notes = Vec::new();
        let total = tracks.len();
        if self.meters > 0 {
            notes.push(format!(
                "`coordinates` read in meters, from the KML standard: OGC 07-147r2 states a \
                 tuple's altitude in meters, and {} of the file's {total} tracks carry one",
                self.meters
            ));
        }
        if self.barometric > 0 {
            notes.push(format!(
                "`coordinates` read in meters as the barometer's altitude above sea level, from \
                 the GPS altitude AltOS writes beside each point: in {} tracks the tuple's altitude \
                 differs from the comment's `alt` at {} of the {} points that carry both, more \
                 than half, as AltOS's KML export has reported both altitudes since AltOS 1.8.3",
                self.barometric, self.barometric_differing, self.barometric_both
            ));
        }
        if self.gps_visualizer > 0 {
            let (first, highest) = extent(tracks.iter().flat_map(|track| track.alt.iter()));
            notes.push(format!(
                "`coordinates` kept as written, as GPS Visualizer takes a source's altitude as \
                 meters unless the source's header names feet: {} tracks carry an altitude, the \
                 highest {:.0} above the first",
                self.gps_visualizer,
                highest - first
            ));
        }
        if !self.ignored.is_empty() {
            let mut modes = self.ignored.clone();
            modes.sort_unstable();
            modes.dedup();
            notes.push(format!(
                "`coordinates` kept as written, in a track whose altitude mode tells a viewer to \
                 ignore its altitude: {} tracks, in mode {}",
                self.ignored.len(),
                modes.join(", ")
            ));
        }
        if self.unnamed > 0 {
            notes.push(format!(
                "`coordinates` kept as written, in an altitude mode the KML standard doesn't \
                 name: {} tracks",
                self.unnamed
            ));
        }
        let (equal, both) = agreement(tracks, Layout::AltOs);
        match self.altos_alt {
            Some(true) => notes.push(format!(
                "`alt` read in meters, from the coordinates: it equals the tuple's altitude, in \
                 meters by the KML standard, at {equal} of the {both} points that carry both"
            )),
            Some(false) => notes.push(format!(
                "`alt` kept as written, as it equals the tuple's altitude at no point, so nothing \
                 in the file shows its unit: {both} points carry both"
            )),
            None => {}
        }
        if self.seconds_alt {
            let (equal, both) = agreement(tracks, Layout::Seconds);
            notes.push(format!(
                "`Alt` kept as written, in a unit the file doesn't state: it equals the tuple's \
                 altitude at {equal} of the {both} points that carry both"
            ));
        }
        notes.extend(clock_notes(tracks, altos_time));
        notes
    }
}

/// The first and highest of a run of values that aren't gaps (0 and 0 if none).
fn extent<'v>(values: impl Iterator<Item = &'v f64>) -> (f64, f64) {
    let mut first = None;
    let mut highest = f64::NEG_INFINITY;
    for &value in values.filter(|value| !value.is_nan()) {
        first.get_or_insert(value);
        highest = highest.max(value);
    }
    first.map_or((0.0, 0.0), |first| (first, highest))
}

/// How a layout's comment altitude agrees with the tuples': at how many points it equals the
/// tuple's, of the points carrying both.
fn agreement(tracks: &[Track<'_>], layout: Layout) -> (usize, usize) {
    let mut both = 0;
    let mut equal = 0;
    for track in tracks {
        for (tuple, remark) in track.alt.iter().zip(&track.remarks) {
            if let Some(remark) = remark
                && remark.layout == layout
                && !remark.alt.is_nan()
                && !tuple.is_nan()
            {
                both += 1;
                equal += usize::from((tuple - remark.alt).abs() <= 1e-6);
            }
        }
    }
    (equal, both)
}

/// The notes deciding each clock's unit, with this file's evidence.
fn clock_notes(tracks: &[Track<'_>], altos_time: AltosTime) -> Vec<String> {
    let mut notes = Vec::new();
    let stamps = || {
        tracks
            .iter()
            .flat_map(|track| track.stamps.iter().flatten())
    };
    let count = |clock: Clock| stamps().filter(|stamp| stamp.clock == clock).count();
    let span = |clock: Clock| {
        let (low, high) = stamps()
            .filter(|stamp| stamp.clock == clock)
            .fold((i64::MAX, i64::MIN), |(low, high), stamp| {
                (low.min(stamp.ns), high.max(stamp.ns))
            });
        if low > high {
            0.0
        } else {
            high.saturating_sub(low) as f64 / NS_PER_S as f64
        }
    };
    let utc = count(Clock::Utc);
    if utc > 0 {
        notes.push(format!(
            "`when` read in UTC, from the KML standard: {utc} times state their zone, `Z` or an \
             offset, as XML Schema's date-time allows"
        ));
    }
    let local = count(Clock::Local);
    if local > 0 {
        notes.push(format!(
            "`when` kept as written, in a zone the file doesn't state: {local} times state none, \
             so the log's clock has an unknown offset from UTC"
        ));
    }
    let (low, high) = (ALTOS_STEP_S.start(), ALTOS_STEP_S.end());
    match altos_time {
        AltosTime::Seconds(median) => notes.push(format!(
            "`time` read in seconds, from the steps between points: the median step between a \
             track's points of distinct times is {median:.2} s, within the {low} to {high} s a \
             GPS receiver reporting once a second gives"
        )),
        AltosTime::Unsettled(median) if median.is_nan() => notes.push(
            "`time` kept as written, as its steps between points don't show its unit: no track \
             holds two points of distinct times, so those points have no time"
                .to_owned(),
        ),
        AltosTime::Unsettled(median) => notes.push(format!(
            "`time` kept as written, as its steps between points don't show its unit: the \
             median step between a track's points of distinct times is {median:.3} read as \
             seconds, outside the {low} to {high} s a GPS receiver reporting once a second \
             gives, so those points have no time"
        )),
        AltosTime::Absent | AltosTime::Unset => {}
    }
    if count(Clock::Seconds) > 0 {
        notes.push(format!(
            "`Seconds` read in seconds, from its name: the points run {:.3} s from first to last",
            span(Clock::Seconds)
        ));
    }
    if count(Clock::TimeOfDay) > 0 {
        notes.push(format!(
            "`UTC` read in hours, minutes and seconds of the UTC day, from its name and layout: \
             the points run {:.0} s from first to last",
            span(Clock::TimeOfDay)
        ));
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-path document around a `coordinates` text.
    fn path(mode: &str, coordinates: &str) -> String {
        format!(
            "<?xml version=\"1.0\"?><kml xmlns=\"http://www.opengis.net/kml/2.2\"><Document>\
             <Placemark><name>Flight</name><LineString>{mode}<coordinates>{coordinates}\
             </coordinates></LineString></Placemark></Document></kml>"
        )
    }

    #[test]
    fn a_kml_root_is_recognized_after_its_prolog() {
        for text in [
            "<kml xmlns=\"http://www.opengis.net/kml/2.2\">",
            "\u{feff}<?xml version=\"1.0\"?>\r\n<!-- a -->\n<!--b--><kml>",
            "  <kml/>",
        ] {
            assert!(recognizes(text), "{text:?}");
        }
        for text in [
            "<kmlx>",
            "<gpx>",
            "<?xml version=\"1.0\"?><gpx><kml/></gpx>",
            "<!-- unclosed <kml>",
            "Time,Altitude\n",
            "",
        ] {
            assert!(!recognizes(text), "{text:?}");
        }
    }

    #[test]
    fn whens_read_with_their_zone() {
        let base = days_from_civil(2026, 5, 23) * DAY_NS;
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2000, 3, 1), 11_017);
        let hour = 3_600 * NS_PER_S;
        let cases = [
            (
                "2026-05-23T10:15:00Z",
                Clock::Utc,
                base + 10 * hour + 900 * NS_PER_S,
            ),
            (
                "2026-05-23T10:15:00.25Z",
                Clock::Utc,
                base + 10 * hour + 900_250_000_000,
            ),
            (
                "2026-05-23T12:15:00+02:00",
                Clock::Utc,
                base + 10 * hour + 900 * NS_PER_S,
            ),
            (
                "2026-05-23T05:15:00-05:00",
                Clock::Utc,
                base + 10 * hour + 900 * NS_PER_S,
            ),
            (
                " 2026-05-23T10:15:00 ",
                Clock::Local,
                base + 10 * hour + 900 * NS_PER_S,
            ),
            (
                "2024-02-29T00:00:00Z",
                Clock::Utc,
                days_from_civil(2024, 2, 29) * DAY_NS,
            ),
        ];
        for (text, clock, ns) in cases {
            assert_eq!(when_ns(text), Some((clock, ns)), "{text}");
        }
        for text in [
            "2026",
            "2026-05",
            "2026-05-23",
            "2026-02-29T00:00:00Z",
            "2026-13-01T00:00:00Z",
            "2026-05-23T24:00:00Z",
            "2026-05-23T10:15Z",
            "2026-05-23T10:15:00+2:00",
            "2026-05-23T10:15:00+15:00",
            "2026-05-23 10:15:00Z",
            "26-05-23T10:15:00Z",
            "2026-05-23T10:15:00.Z",
            "2026-05-23T10:15:00.5e3Z",
            "",
        ] {
            assert_eq!(when_ns(text), None, "{text}");
        }
    }

    #[test]
    fn decimals_read_exactly_to_the_nanosecond() {
        assert_eq!(decimal_ns("123.4500000"), Some(123_450_000_000));
        assert_eq!(decimal_ns("-0.25"), Some(-250_000_000));
        assert_eq!(decimal_ns("1"), Some(NS_PER_S));
        assert_eq!(decimal_ns(".5"), Some(500_000_000));
        assert_eq!(decimal_ns("0.1234567899"), Some(123_456_789));
        for text in ["", "-", ".", "1e3", "1.2.3", "nan", "12345678901", "+-1"] {
            assert_eq!(decimal_ns(text), None, "{text}");
        }
        assert_eq!(time_of_day_ns("12:34:56"), Some(45_296 * NS_PER_S));
        assert_eq!(time_of_day_ns("00:00:00.5"), Some(NS_PER_S / 2));
        for text in [
            "24:00:00",
            "1:00:00",
            "12:60:00",
            "12:00:61",
            "12:00",
            "12:00:00:00",
        ] {
            assert_eq!(time_of_day_ns(text), None, "{text}");
        }
    }

    #[test]
    fn tuples_take_two_or_three_numbers_whatever_the_spacing() {
        let read: Vec<Option<(f64, f64, Option<f64>)>> =
            tuples(" -97.1,35.2,402\n\t-97.1, 35.2 ,  415\r\n-97.2,35.3 -97.3,35.4,-0.5 ")
                .iter()
                .map(|raw| tuple(raw, false))
                .collect();
        assert_eq!(
            read,
            [
                Some((-97.1, 35.2, Some(402.0))),
                Some((-97.1, 35.2, Some(415.0))),
                Some((-97.2, 35.3, None)),
                Some((-97.3, 35.4, Some(-0.5))),
            ]
        );
        assert_eq!(
            tuple("-97.1,35.2,00850", false),
            Some((-97.1, 35.2, Some(850.0)))
        );
        for raw in [
            "-97.1",
            "-97.1,35.2,1,2",
            "-97.1,,1",
            "a,b",
            "-97.1,35.2,",
            "1,inf",
        ] {
            assert_eq!(tuple(raw, false), None, "{raw}");
        }
        assert_eq!(
            tuple(" -97.1  35.2 402.5 ", true),
            Some((-97.1, 35.2, Some(402.5)))
        );
        assert_eq!(tuple("-97.1 35.2", true), Some((-97.1, 35.2, None)));
        for raw in ["", "-97.1,35.2,402", "1 2 3 4"] {
            assert_eq!(tuple(raw, true), None, "{raw}");
        }
    }

    #[test]
    fn each_comment_layout_reads() {
        let altos = remark(" alt    7.5000000 time   -0.2500000 sats 5 ").unwrap();
        assert_eq!(
            (altos.layout, altos.alt, altos.time, altos.sats),
            (Layout::AltOs, 7.5, Some((-250_000_000, "-0.2500000")), 5.0)
        );
        assert!(altos.offset.is_nan());
        let seconds = remark(" Alt 250.125 Seconds 9.5 ").unwrap();
        assert_eq!(
            (seconds.layout, seconds.alt, seconds.time),
            (Layout::Seconds, 250.125, Some((9_500_000_000, "9.5")))
        );
        assert!(seconds.sats.is_nan());
        let beeline = remark(" 1003 sats:8 UTC 12:34:56 ").unwrap();
        assert_eq!(
            (beeline.layout, beeline.offset, beeline.sats, beeline.time),
            (
                Layout::BeeLine,
                1_003.0,
                8.0,
                Some((45_296 * NS_PER_S, "12:34:56"))
            )
        );
        assert!(beeline.alt.is_nan());
        for text in [
            " Name       : a rocket ",
            " Capture Date: 01/02/2030 ",
            " base    : 0m or     0 feet ",
            " alt 9 time x sats 5 ",
            " alt 9 time 1 sats 5 extra ",
            " 1 sats:x UTC 12:34:56 ",
            " 1 sats:4 UTC 25:00:00 ",
            "",
        ] {
            assert_eq!(remark(text), None, "{text:?}");
        }
    }

    #[test]
    fn a_comment_belongs_to_the_tuple_right_before_it() {
        let log = read(&path(
            "<altitudeMode>absolute</altitudeMode>",
            "<!-- header -->\n-97.1,35.2,400 <!-- alt 401 time 0.5 sats 4 -->\n\
             -97.1,35.2,410 <!-- alt 411 time 1.5 sats 5 --> <!-- second -->\n-97.1,35.2,420",
        ))
        .unwrap();
        let columns: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
        assert_eq!(
            columns,
            [
                "Flight latitude",
                "Flight longitude",
                "Flight altitude",
                "Flight GPS altitude",
                "Flight satellites"
            ]
        );
        let gps = &log.channels[3];
        assert_eq!(gps.values[..2], [401.0, 411.0]);
        assert!(gps.values[2].is_nan());
        assert_eq!(gps.time_s[..2], [0.0, 1.0]);
        assert!(gps.time_s[2].is_nan());
        assert_eq!(log.channels[2].quantity, Quantity::AltitudeMsl);
        // `header` precedes every tuple and `second` follows a comment: neither is a point's.
        for expected in [
            "2 comments in `coordinates` follow no point's",
            "1 of the file's 3 points",
        ] {
            assert!(
                log.notes.iter().any(|note| note.contains(expected)),
                "{expected}: {:?}",
                log.notes
            );
        }
    }

    #[test]
    fn the_altitude_mode_sets_the_quantity() {
        let quantity = |mode: &str| {
            let log = read(&path(mode, "-97.1,35.2,400 -97.1,35.2,410")).unwrap();
            (log.channels[2].column.clone(), log.channels[2].quantity)
        };
        assert_eq!(
            quantity("<altitudeMode>absolute</altitudeMode>"),
            ("Flight altitude".to_owned(), Quantity::GnssAltitude)
        );
        assert_eq!(
            quantity("<altitudeMode>relativeToGround</altitudeMode>"),
            ("Flight height above ground".to_owned(), Quantity::Distance)
        );
        assert_eq!(
            quantity(
                "<gx:altitudeMode xmlns:gx=\"http://www.google.com/kml/ext/2.2\">relativeToSeaFloor</gx:altitudeMode>"
            ),
            (
                "Flight height above sea floor".to_owned(),
                Quantity::Distance
            )
        );
        for mode in [
            "",
            "<altitudeMode>clampToGround</altitudeMode>",
            "<altitudeMode>up</altitudeMode>",
        ] {
            assert_eq!(
                quantity(mode),
                ("Flight altitude".to_owned(), Quantity::Unrecognized),
                "{mode}"
            );
        }
        // Two numbers a tuple: no altitude channel.
        let log = read(&path("", "-97.1,35.2 -97.1,35.3")).unwrap();
        assert_eq!(log.channels.len(), 2);
    }

    #[test]
    fn concatenated_documents_split_at_each_declaration() {
        let one = path("<altitudeMode>absolute</altitudeMode>", "-97.1,35.2,400");
        let twice = format!("{one}\n{one}\n");
        let (pieces, more) = split_documents(&twice, MAX_NODES).unwrap();
        assert_eq!(pieces.len(), 2);
        assert!(!more);
        assert!(pieces.iter().all(|piece| piece.trim() == one));
        let log = read(&format!("{one}\n{one}\n{one}")).unwrap();
        assert_eq!(log.channels.len(), 3);
        assert!(
            log.notes.iter().any(|n| n.contains("3 times over")),
            "{:?}",
            log.notes
        );
        let other = path("<altitudeMode>absolute</altitudeMode>", "-97.2,35.3,500");
        let log = read(&format!("{one}{other}")).unwrap();
        let columns: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
        assert_eq!(columns[3], "Flight (2) latitude");
        assert!(log.notes.iter().any(|n| n.contains("1 more KML documents")));
        // Trailing text that isn't a document is refused, not skipped.
        assert!(matches!(
            read(&format!("{one}<kml/>")),
            Err(LogError::Syntax { .. })
        ));
    }

    #[test]
    fn hostile_documents_are_refused() {
        let dtd = "<?xml version=\"1.0\"?><!DOCTYPE kml [<!ENTITY a \"aaaa\">]><kml>&a;</kml>";
        let error = read(dtd).unwrap_err();
        assert!(error.to_string().contains("document type"), "{error}");
        assert!(matches!(
            read("<kml><Placemark>"),
            Err(LogError::Syntax { .. })
        ));
        assert!(matches!(read("<kml/>"), Err(LogError::NoData { .. })));
        assert!(matches!(
            read("<gpx/>"),
            Err(LogError::NotThisFormat { .. })
        ));
    }

    #[test]
    fn tracks_past_the_cap_are_counted() {
        let pins: String = (0..MAX_TRACKS + 3)
            .map(|index| {
                format!("<Placemark><Point><coordinates>-100,40,{index}</coordinates></Point></Placemark>")
            })
            .collect();
        let log = read(&format!("<kml><Document>{pins}</Document></kml>")).unwrap();
        // Latitude, longitude and an altitude kept as written, no mode being stated.
        assert_eq!(log.channels.len(), 3 * MAX_TRACKS);
        assert_eq!(log.channels[3].column, "track (2) latitude");
        assert!(
            log.notes
                .iter()
                .any(|note| note.starts_with("the file holds 3 more geometries than the 256")),
            "{:?}",
            log.notes
        );
    }

    #[test]
    fn a_document_type_in_the_prolog_is_stepped_past() {
        for text in [
            "<?xml version=\"1.0\"?><!DOCTYPE kml><kml/>",
            "<!DOCTYPE kml [<!ENTITY a \"x>y\"> <!ENTITY b 'z]'>]>\n<!-- c --><kml>&a;</kml>",
        ] {
            assert!(recognizes(text), "{text:?}");
        }
        for text in ["<!DOCTYPE kml [<!ENTITY a \"x\">", "<!DOCTYPE gpx><gpx/>"] {
            assert!(!recognizes(text), "{text:?}");
        }
    }

    #[test]
    fn nesting_is_counted_past_comments_quotes_and_closed_tags() {
        assert_eq!(
            nesting("<a><b/><c x='>' y=\"<d>\"><!-- <e> --></c></a>"),
            (2, 0)
        );
        assert_eq!(nesting("<a><![CDATA[<b><c>]]><?p <q>?></a>"), (1, 0));
        let deep = format!("<k>\n{}", "<a>".repeat(MAX_DEPTH));
        assert_eq!(nesting(&deep), (MAX_DEPTH + 1, 2));
        let error = read(&deep).unwrap_err();
        assert!(
            matches!(&error, LogError::Syntax { line: 2, message, .. }
                if message.contains(&format!("nests elements {} deep", MAX_DEPTH + 1))),
            "{error:?}"
        );
    }

    #[test]
    fn the_node_limit_is_the_files_over_all_its_documents() {
        let one = path("<altitudeMode>absolute</altitudeMode>", "-97.1,35.2,400");
        let other = path("<altitudeMode>absolute</altitudeMode>", "-97.2,35.3,500");
        let nodes = u32::try_from(Document::parse(&one).unwrap().descendants().count()).unwrap();
        // Each document fits alone; the two together don't, nor do they one node short.
        assert!(read_within(&one, nodes).is_ok());
        assert!(read_within(&other, nodes).is_ok());
        let both = format!("{one}{other}");
        for budget in [nodes, nodes + 1, 2 * nodes - 1] {
            let error = read_within(&both, budget).unwrap_err();
            assert!(
                error.to_string().contains("XML nodes over its documents"),
                "{budget}: {error}"
            );
        }
        assert!(read_within(&both, 2 * nodes).is_ok());
        // A copy of the first document isn't parsed again, so it takes nothing from the budget.
        assert!(read_within(&format!("{one}\n{one}"), nodes).is_ok());
    }

    /// A BeeLine path whose comments give its points these times of day, in turn.
    fn beeline<'t>(times: impl Iterator<Item = &'t str>) -> String {
        let coordinates: String = times
            .enumerate()
            .map(|(index, time)| format!("-100,40,{index} <!-- {index} sats:8 UTC {time} -->\n"))
            .collect();
        path("<altitudeMode>absolute</altitudeMode>", &coordinates)
    }

    #[test]
    fn midnight_is_passed_at_most_max_midnights_times() {
        let alternate = |count| beeline(["23:00:00", "00:00:00"].into_iter().cycle().take(count));
        let day_s = 86_400.0;
        // 15 points pass midnight 7 times, the bound: every point keeps its time.
        let log = read(&alternate(15)).unwrap();
        let time = &log.channels[0].time_s;
        assert!(time.iter().all(|time| time.is_finite()), "{time:?}");
        assert_eq!(time[13], 7.0 * day_s - 23.0 * 3_600.0);
        assert_eq!(time[14], 7.0 * day_s);
        assert!(
            log.notes
                .iter()
                .any(|note| note.contains("midnight 7 times"))
        );
        // A 16th point would pass it an 8th time: its time is a gap, with a note.
        let log = read(&alternate(16)).unwrap();
        assert!(log.channels[0].time_s[15].is_nan());
        assert!(log.channels[0].time_s[..15].iter().all(|t| t.is_finite()));
        assert!(
            log.notes.iter().any(|note| note.starts_with(
                "the time of day steps back past midnight more than 7 times, more than a \
                 logger's memory holds: the 1 times of day from there on are gaps"
            )),
            "{:?}",
            log.notes
        );
        // 230,000 points stepping back at every other one: no day count overflows, and every
        // time past the bound is a gap.
        let log = read(&alternate(230_000)).unwrap();
        let time = &log.channels[0].time_s;
        assert_eq!(time.iter().filter(|time| time.is_finite()).count(), 15);
        assert!(
            log.notes
                .iter()
                .any(|note| note.contains("the 229985 times of day from there on are gaps")),
            "{:?}",
            log.notes
        );
    }

    /// A three-point AltOS path, the tuples' altitudes 100, 200 and 300, with these comment
    /// altitudes and times.
    fn altos(alts: [&str; 3], times: [&str; 3]) -> String {
        let coordinates: String = (0..3)
            .map(|index| {
                format!(
                    "-100,40,{} <!-- alt {} time {} sats 5 -->\n",
                    100 * (index + 1),
                    alts[index],
                    times[index]
                )
            })
            .collect();
        path("<altitudeMode>absolute</altitudeMode>", &coordinates)
    }

    /// The quantity of a log's channel by its column, if it has one.
    fn quantity_of(log: &FlightLog, column: &str) -> Option<Quantity> {
        log.channels
            .iter()
            .find(|channel| channel.column == column)
            .map(|channel| channel.quantity)
    }

    /// Whether a note opens with the text.
    fn noted(log: &FlightLog, opening: &str) -> bool {
        log.notes.iter().any(|note| note.starts_with(opening))
    }

    #[test]
    fn an_altos_track_is_the_barometers_where_most_of_its_points_differ() {
        let seconds = ["1", "2", "3"];
        let barometric = "`coordinates` read in meters as the barometer's altitude above sea \
                          level, from the GPS altitude AltOS writes beside each point: ";
        // One point of three differs: the tuple's is the GPS altitude, and the comment's a
        // channel in meters, as it equals the tuple's elsewhere.
        let log = read(&altos(["100", "200", "333"], seconds)).unwrap();
        assert_eq!(
            quantity_of(&log, "Flight altitude"),
            Some(Quantity::GnssAltitude)
        );
        assert_eq!(
            quantity_of(&log, "Flight GPS altitude"),
            Some(Quantity::GnssAltitude)
        );
        assert!(!noted(&log, barometric), "{:?}", log.notes);
        assert!(noted(
            &log,
            "`alt` read in meters, from the coordinates: it equals the tuple's altitude, in meters \
             by the KML standard, at 2 of the 3 points that carry both"
        ));
        // Two of three differ: the tuple's is the barometer's.
        let log = read(&altos(["100", "222", "333"], seconds)).unwrap();
        assert_eq!(
            quantity_of(&log, "Flight altitude"),
            Some(Quantity::AltitudeMsl)
        );
        assert_eq!(
            quantity_of(&log, "Flight GPS altitude"),
            Some(Quantity::GnssAltitude)
        );
        assert!(
            noted(
                &log,
                &format!(
                    "{barometric}in 1 tracks the tuple's altitude differs from the comment's \
                     `alt` at 2 of the 3 points that carry both"
                )
            ),
            "{:?}",
            log.notes
        );
        // None equal: the comment's altitude shows no unit, and is kept as written.
        let log = read(&altos(["111", "222", "333"], seconds)).unwrap();
        assert_eq!(
            quantity_of(&log, "Flight GPS altitude"),
            Some(Quantity::Unrecognized)
        );
        assert!(noted(
            &log,
            "`alt` kept as written, as it equals the tuple's altitude at no point, so nothing in \
             the file shows its unit: 3 points carry both"
        ));
        // All equal: the comment's altitude adds nothing.
        let log = read(&altos(["100", "200", "300"], seconds)).unwrap();
        assert_eq!(
            quantity_of(&log, "Flight altitude"),
            Some(Quantity::GnssAltitude)
        );
        assert_eq!(quantity_of(&log, "Flight GPS altitude"), None);
        assert!(!log.notes.iter().any(|note| note.starts_with("`alt`")));
    }

    #[test]
    fn altos_time_is_read_in_seconds_only_where_its_steps_are_about_a_second() {
        let alts = ["100", "200", "300"];
        let log = read(&altos(alts, ["10", "11", "12"])).unwrap();
        assert_eq!(log.channels[0].time_s, [0.0, 1.0, 2.0]);
        assert_eq!(quantity_of(&log, "Flight time"), None);
        assert!(noted(
            &log,
            "`time` read in seconds, from the steps between points: the median step between a \
             track's points of distinct times is 1.00 s, within the 0.5 to 2 s"
        ));
        // The bounds read; just past them, and steps of 100, don't.
        for (times, seconds) in [
            (["0", "0.5", "1"], true),
            (["0", "2", "4"], true),
            (["0", "0.49", "0.98"], false),
            (["0", "2.01", "4.02"], false),
            (["100", "200", "300"], false),
        ] {
            let log = read(&altos(alts, times)).unwrap();
            let timed = log.channels[0].time_s.iter().all(|time| time.is_finite());
            assert_eq!(timed, seconds, "{times:?}");
            assert_eq!(
                quantity_of(&log, "Flight time").is_none(),
                seconds,
                "{times:?}"
            );
        }
        // Kept as written: a channel of its own, no time, and a note.
        let log = read(&altos(alts, ["100", "200", "300"])).unwrap();
        assert!(log.channels[0].time_s.iter().all(|time| time.is_nan()));
        let time = log
            .channels
            .iter()
            .find(|channel| channel.column == "Flight time")
            .unwrap();
        assert_eq!(
            (time.quantity, &time.values[..]),
            (Quantity::Unrecognized, &[100.0, 200.0, 300.0][..])
        );
        assert!(noted(
            &log,
            "`time` kept as written, as its steps between points don't show its unit: the median \
             step between a track's points of distinct times is 100.000 read as seconds"
        ));
        // A single point has no step to show it.
        let log = read(&path(
            "<altitudeMode>absolute</altitudeMode>",
            "-100,40,100 <!-- alt 100 time 5 sats 5 -->",
        ))
        .unwrap();
        assert!(noted(
            &log,
            "`time` kept as written, as its steps between points don't show its unit: no track \
             holds two points of distinct times"
        ));
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has a time and a value for
        /// each sample on every channel, and events at finite times.
        #[test]
        fn any_text_is_read_or_refused(
            mode in "(absolute|relativeToGround|clampToGround|x|)",
            coordinates in "[-0-9.,:\\s]{0,60}",
            comment in "(alt|Alt|time|Seconds|sats|sats:|UTC|[0-9.:-]| ){0,40}",
            when in "[-0-9:TZ.+]{0,30}",
            raw in "\\PC{0,80}",
        ) {
            let texts = [
                path(&format!("<altitudeMode>{mode}</altitudeMode>"), &format!("{coordinates}<!--{comment}-->{coordinates}")),
                format!("<kml><Placemark><name>p</name><gx:Track xmlns:gx=\"g\"><when>{when}</when><gx:coord>{coordinates}</gx:coord></gx:Track></Placemark></kml>"),
                format!("<kml><Placemark><TimeStamp><when>{when}</when></TimeStamp><Point><coordinates>{coordinates}</coordinates></Point></Placemark></kml>"),
                format!("<kml>{raw}</kml>"),
                raw.clone(),
            ];
            for text in texts {
                if let Ok(log) = read(&text) {
                    proptest::prop_assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
                    for channel in &log.channels {
                        proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                    }
                    for event in &log.events {
                        proptest::prop_assert!(event.time_s.is_finite());
                    }
                }
            }
        }
    }
}
