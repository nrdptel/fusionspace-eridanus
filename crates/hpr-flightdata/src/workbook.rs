//! Excel workbooks, `.xlsx`: a flight's log as a flyer saved it from a spreadsheet, one sheet or
//! several, such as a Blue Raven's high-rate and low-rate files side by side.
//!
//! A workbook isn't a layout of its own. [`sheets`] writes each worksheet's cells out as
//! comma-separated text, one line a row, and [`read`] gives each sheet's text to
//! [`import::read`], which recognizes it as it would the file the sheet
//! came from: a Blue Raven's CSV, an Eggtimer's, a Featherweight tracker's, or a table read by
//! the units its header states ([`generic`](crate::generic)). One workbook can give several logs.
//!
//! An `.xlsx` file is a zip archive of XML parts (ECMA-376, *Office Open XML File Formats*, 5th
//! edition, Part 1, §18 *SpreadsheetML*), taken out in memory as [`archive::unpack`] takes any
//! archive's files, within its bound. The package's relationships (Part 2, §9) lead to the
//! workbook part, and the workbook's to its worksheets, shared strings and styles.
//!
//! **What a cell becomes.** A cell is written as the workbook stores it, never as a spreadsheet
//! would display it, so no digit is lost or invented:
//!
//! | The cell | Written as |
//! |---|---|
//! | a number (§18.3.1.4, no `t` or `t="n"`) | its stored digits, such as `0.10000000000000001` |
//! | a shared string (`t="s"`, §18.4.8) | the string's text, every run's, a phonetic guide's left out |
//! | an inline string (`t="inlineStr"`) or a formula's text (`t="str"`) | its text |
//! | a formula | the value saved with it; none saved, a gap |
//! | an error (`t="e"`), such as `#DIV/0!` | a gap |
//! | a truth value (`t="b"`) | `TRUE` or `FALSE` |
//! | an ISO 8601 date (`t="d"`) | its text |
//! | a number under a date or time format (§18.8.30, §18.8.31) | the date and time it stands for, below |
//!
//! A cell whose text holds a comma or a double quote is quoted, its quotes doubled; a line break
//! inside a cell, as in a header wrapped to two lines, becomes a space.
//!
//! **Dates and times.** A spreadsheet stores a date as a count of days, a serial number, and
//! shows it as a date only through the cell's number format. A number under a format that shows
//! a date is written `2023-03-15`, under one that shows a time of day `18:00:00`, under one that
//! shows both `2023-03-15 18:00:00`, and under an elapsed-time format such as `[h]:mm:ss`, which
//! counts hours past 24, `36:00:00`. The seconds are rounded to the millisecond and their fraction
//! written only when it isn't zero, as `09.375`; a serial's double holds a time of day in 2023 to
//! about a microsecond. Days count from 1899-12-30 (so serial 1 is 1900-01-01, past Excel's
//! 1900-02-29, a day that never was), or from 1904-01-01 in a workbook that says so
//! (`date1904`, §18.2.28). A date before 1900-03-01, where the two calendars disagree, or past
//! 9999-12-31 is kept as the number stored, with a note. A format shows a date when its first
//! section, outside quotes, brackets and escapes, holds a `y`, a `d` or an `m` for the month, and
//! a time when it holds an `h`, an `s` or an `m` for the minute: an `m` right after an `h` or
//! right before an `s` is the minute (§18.8.31). Every other number is kept as stored, so a time
//! a logger wrote as seconds stays a number.
//!
//! **Rows and columns.** Cells are placed by their references (`B12`). Every row is written as
//! wide as the sheet's last column that holds a value, so a short row gets empty cells and a
//! cell formatted but empty widens nothing; a row with no value is left out. A sheet with no
//! value, such as one that only holds a chart, gives no text, and [`read`] says it has no data.
//! Chart sheets hold no cells and aren't listed.
//!
//! **Bounds.** A workbook comes from someone else, so its text is bounded: at most
//! [`MAX_TEXT_BYTES`] of written text from one workbook, counted as each cell is read, and at
//! most [`MAX_SMALL_PART_BYTES`] in each part read whole (relationships, the workbook's list of
//! sheets, the styles). The sheets and shared strings, which grow with the data, are read as a
//! stream (the largest sample sheet is 60 MB of XML). A document type declaration is refused, so
//! no entity can expand, and every reference but XML's own five and characters' is refused.
//! Cells and rows must come in order, as §18.3.1.4 and §18.3.1.73 have them, two sheets may not
//! share a part, and a reference past `XFD1048576`, Excel's last cell, is refused.
//!
//! ```
//! use std::io::{Cursor, Write};
//!
//! use hpr_flightdata::{log::LogFormat, workbook};
//!
//! // An invented workbook of one sheet: a header naming its units, then two rows.
//! let parts = [
//!     ("_rels/.rels", r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="r1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#),
//!     ("xl/workbook.xml", r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Flight" sheetId="1" r:id="s1"/></sheets></workbook>"#),
//!     ("xl/_rels/workbook.xml.rels", r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="s1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#),
//!     ("xl/worksheets/sheet1.xml", r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>
//!         <row r="1"><c r="A1" t="inlineStr"><is><t>Time (s)</t></is></c><c r="B1" t="inlineStr"><is><t>Altitude (m)</t></is></c></row>
//!         <row r="2"><c r="A2"><v>0</v></c><c r="B2"><v>0</v></c></row>
//!         <row r="3"><c r="A3"><v>0.05</v></c><c r="B3"><v>1.5</v></c></row>
//!     </sheetData></worksheet>"#),
//! ];
//! let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
//! for (name, xml) in parts {
//!     zip.start_file(name, zip::write::SimpleFileOptions::default())?;
//!     zip.write_all(xml.as_bytes())?;
//! }
//! let bytes = zip.finish()?.into_inner();
//!
//! let sheets = workbook::sheets(&bytes)?;
//! assert_eq!(sheets[0].text, "Time (s),Altitude (m)\n0,0\n0.05,1.5\n");
//! let logs = workbook::read(&bytes)?;
//! assert_eq!(logs[0].log.as_ref().map(|log| log.format), Ok(LogFormat::GenericTable));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};
use serde::{Deserialize, Serialize};

use crate::archive::{self, Member};
use crate::error::LogError;
use crate::import;
use crate::log::FlightLog;

/// The most text [`sheets`] writes from one workbook, 256 MiB (268 MB), counted as each cell is
/// read: the bound [`archive::MAX_UNPACKED_BYTES`] puts on the parts it is written from.
pub const MAX_TEXT_BYTES: usize = 256 * 1024 * 1024;

/// The most bytes in a part read whole as a tree, 16 MiB (17 MB): the relationships, the
/// workbook's list of sheets and the styles, the largest of them in the project's sample
/// workbooks well under a megabyte.
pub const MAX_SMALL_PART_BYTES: usize = 16 * 1024 * 1024;

/// Excel's last column, `XFD` (§18.3.1.4 leaves the limit to the application; Excel's is 16,384).
const MAX_COLUMNS: u32 = 16_384;

/// Excel's last row, 1,048,576.
const MAX_ROWS: u32 = 1_048_576;

/// The format's name in errors.
const FORMAT: &str = "Excel workbook";

/// SpreadsheetML's main namespace, Transitional and Strict (ECMA-376 Part 1, §18; Part 1, Annex
/// A of the Strict schemas).
const MAIN_NAMESPACES: [&str; 2] = [
    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    "http://purl.oclc.org/ooxml/spreadsheetml/main",
];

/// One worksheet of a workbook, its cells written out as text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Sheet {
    /// The sheet's name, as its tab shows it.
    pub name: String,
    /// The sheet's cells, comma-separated, one line a row, each line ending in `\n`; empty for a
    /// sheet with no value.
    pub text: String,
    /// What the text doesn't show as the workbook stores it, each a count: errors and formulas
    /// with no saved value written as gaps, dates kept as numbers.
    pub notes: Vec<String>,
}

/// One worksheet of a workbook, read.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SheetLog {
    /// The sheet's name, as its tab shows it.
    pub name: String,
    /// The log its text reads as, its notes opening with the sheet it was read from; or why it
    /// doesn't read, [`LogError::NoData`] for a sheet with no value.
    pub log: Result<FlightLog, LogError>,
}

/// Each worksheet of an `.xlsx` workbook, read as a log on its own by
/// [`import::read`], in the workbook's order of sheets.
///
/// # Errors
///
/// As [`sheets`]; a sheet that doesn't read is not an error, but its [`SheetLog::log`].
pub fn read(bytes: &[u8]) -> Result<Vec<SheetLog>, LogError> {
    Ok(sheets(bytes)?
        .into_iter()
        .map(|sheet| {
            let log = if sheet.text.is_empty() {
                Err(LogError::NoData { format: FORMAT })
            } else {
                import::read(sheet.text.as_bytes()).map(|mut log| {
                    let mut notes = vec![format!(
                        "read from the sheet {:?} of an Excel workbook, its cells as stored",
                        shortened(&sheet.name)
                    )];
                    notes.extend(sheet.notes);
                    notes.append(&mut log.notes);
                    log.notes = notes;
                    log
                })
            };
            SheetLog {
                name: sheet.name,
                log,
            }
        })
        .collect())
}

/// Each worksheet of an `.xlsx` workbook as text; [`sheets_within`] with [`MAX_TEXT_BYTES`].
///
/// # Errors
///
/// As [`sheets_within`].
pub fn sheets(bytes: &[u8]) -> Result<Vec<Sheet>, LogError> {
    sheets_within(bytes, MAX_TEXT_BYTES)
}

/// Each worksheet of an `.xlsx` workbook as text, in the workbook's order of sheets, writing at
/// most `max_text_bytes` from the whole workbook.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] when the bytes aren't a zip archive, or the archive holds no
/// SpreadsheetML workbook (a word processor's `.docx`, say); [`LogError::Archive`] as
/// [`archive::unpack`]; [`LogError::Workbook`] when a part the workbook names is missing or isn't
/// well-formed XML, holds a document type declaration or a reference to an entity, cells or rows
/// come out of order or past `XFD1048576`, a cell's value isn't what its type says (a number that
/// isn't one, a shared string that isn't there), two sheets share a part, a part read whole is
/// larger than [`MAX_SMALL_PART_BYTES`], or the text would pass `max_text_bytes`.
pub fn sheets_within(bytes: &[u8], max_text_bytes: usize) -> Result<Vec<Sheet>, LogError> {
    if !archive::is_archive(bytes) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "its bytes don't open as a zip archive's, as an `.xlsx` file's do".to_owned(),
        });
    }
    let package = Package::new(archive::unpack(bytes)?)?;
    let main = package
        .relationships("")?
        .into_iter()
        .find(|rel| rel.kind == "officeDocument" && !rel.external)
        .ok_or_else(|| LogError::NotThisFormat {
            format: FORMAT,
            message: "the archive's relationships (`_rels/.rels`) name no office document"
                .to_owned(),
        })?;
    let book = Book::read(&package, &main.target)?;
    let mut budget = Budget {
        left: max_text_bytes,
        most: max_text_bytes,
    };
    let mut used_parts = HashSet::new();
    let mut out = Vec::new();
    for (name, target) in &book.sheets {
        if !used_parts.insert(target.to_ascii_lowercase()) {
            return Err(refused(format!(
                "the sheets {:?} and another share the part {:?}",
                shortened(name),
                shortened(target)
            )));
        }
        let xml = package.text(target)?;
        let (text, notes) = read_sheet(&xml, target, &book, &mut budget)?;
        out.push(Sheet {
            name: name.clone(),
            text,
            notes,
        });
    }
    Ok(out)
}

/// A refusal of the workbook's content.
fn refused(message: String) -> LogError {
    LogError::Workbook { message }
}

/// The text still allowed out of the workbook.
struct Budget {
    left: usize,
    most: usize,
}

impl Budget {
    /// Takes `bytes` from what is left, or refuses the workbook.
    fn take(&mut self, bytes: usize) -> Result<(), LogError> {
        self.left = self.left.checked_sub(bytes).ok_or_else(|| {
            refused(format!(
                "its sheets come to more than {} bytes of text, the most written from one workbook",
                self.most
            ))
        })?;
        Ok(())
    }
}

/// The parts of the package, by name; a part's name compares without case (ECMA-376 Part 2,
/// §6.2.2.3).
struct Package {
    members: Vec<Member>,
    by_name: HashMap<String, usize>,
}

impl Package {
    fn new(members: Vec<Member>) -> Result<Self, LogError> {
        let mut by_name = HashMap::new();
        for (index, member) in members.iter().enumerate() {
            let name = member.name.trim_start_matches('/').to_ascii_lowercase();
            if by_name.insert(name, index).is_some() {
                return Err(refused(format!(
                    "two parts are named {:?}, apart from case",
                    shortened(&member.name)
                )));
            }
        }
        Ok(Self { members, by_name })
    }

    fn get(&self, name: &str) -> Option<&[u8]> {
        self.by_name
            .get(&name.to_ascii_lowercase())
            .map(|&index| self.members[index].bytes.as_slice())
    }

    /// A part the workbook names, as text.
    fn text(&self, name: &str) -> Result<Cow<'_, str>, LogError> {
        self.get(name)
            .map(import::decode)
            .ok_or_else(|| refused(format!("the part {:?} is missing", shortened(name))))
    }

    /// A part read whole as a tree, within [`MAX_SMALL_PART_BYTES`]; `None` if there is none.
    fn small(&self, name: &str) -> Result<Option<String>, LogError> {
        let Some(bytes) = self.get(name) else {
            return Ok(None);
        };
        if bytes.len() > MAX_SMALL_PART_BYTES {
            return Err(refused(format!(
                "the part {:?} is {} bytes, more than the {MAX_SMALL_PART_BYTES} read whole",
                shortened(name),
                bytes.len()
            )));
        }
        Ok(Some(import::decode(bytes).into_owned()))
    }

    /// The relationships of the part `source` (the package's own for `""`), each target resolved
    /// to a part's name (ECMA-376 Part 2, §9.3); none if it has no relationships part.
    fn relationships(&self, source: &str) -> Result<Vec<Relationship>, LogError> {
        let (folder, file) = match source.rsplit_once('/') {
            Some((folder, file)) => (folder, file),
            None => ("", source),
        };
        let rels = if folder.is_empty() {
            format!("_rels/{file}.rels")
        } else {
            format!("{folder}/_rels/{file}.rels")
        };
        let Some(xml) = self.small(&rels)? else {
            return Ok(Vec::new());
        };
        let doc = tree(&xml, &rels)?;
        let mut out = Vec::new();
        for node in doc.root_element().children().filter(|node| {
            node.has_tag_name("Relationship") || node.tag_name().name() == "Relationship"
        }) {
            let (Some(id), Some(kind), Some(target)) = (
                node.attribute("Id"),
                node.attribute("Type"),
                node.attribute("Target"),
            ) else {
                return Err(refused(format!(
                    "a relationship in {rels:?} lacks its `Id`, `Type` or `Target`"
                )));
            };
            let external = node.attribute("TargetMode") == Some("External");
            out.push(Relationship {
                id: id.to_owned(),
                // The type's last segment: the Transitional and Strict URIs differ before it.
                kind: kind.rsplit('/').next().unwrap_or(kind).to_owned(),
                target: if external {
                    target.to_owned()
                } else {
                    resolve(folder, target)
                },
                external,
            });
        }
        Ok(out)
    }
}

/// A relationship from one part to another.
struct Relationship {
    id: String,
    kind: String,
    target: String,
    external: bool,
}

/// A relationship's target as a part's name: from the package's root if it starts with `/`,
/// else from the source's folder, `.` and `..` resolved.
fn resolve(folder: &str, target: &str) -> String {
    let mut parts: Vec<&str> = if target.starts_with('/') {
        Vec::new()
    } else {
        folder.split('/').filter(|part| !part.is_empty()).collect()
    };
    for part in target.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part),
        }
    }
    parts.join("/")
}

/// A small part as a tree; a document type declaration is refused (roxmltree's default).
fn tree<'a>(xml: &'a str, name: &str) -> Result<roxmltree::Document<'a>, LogError> {
    roxmltree::Document::parse(xml).map_err(|error| {
        refused(format!(
            "the part {:?} isn't well-formed XML: {error}",
            shortened(name)
        ))
    })
}

/// The workbook part's sheets, the shared strings and each style's date kind.
struct Book {
    /// Each worksheet's name and part, in the workbook's order.
    sheets: Vec<(String, String)>,
    strings: Vec<String>,
    /// Each cell format's date kind (`cellXfs`, §18.8.10), by the index a cell's `s` names.
    styles: Vec<Option<DateKind>>,
    date1904: bool,
}

impl Book {
    fn read(package: &Package, part: &str) -> Result<Self, LogError> {
        let xml = package.small(part)?.ok_or_else(|| {
            refused(format!(
                "the workbook part {:?} is missing",
                shortened(part)
            ))
        })?;
        let doc = tree(&xml, part)?;
        let root = doc.root_element();
        if root.tag_name().name() != "workbook"
            || !root
                .tag_name()
                .namespace()
                .is_some_and(|uri| MAIN_NAMESPACES.contains(&uri))
        {
            return Err(LogError::NotThisFormat {
                format: FORMAT,
                message: "the office document isn't a SpreadsheetML workbook".to_owned(),
            });
        }
        let rels = package.relationships(part)?;
        let date1904 = root
            .children()
            .find(|node| node.tag_name().name() == "workbookPr")
            .and_then(|node| node.attribute("date1904"))
            .is_some_and(|value| matches!(value, "1" | "true"));
        let mut sheets = Vec::new();
        for sheet in root
            .children()
            .filter(|node| node.tag_name().name() == "sheets")
            .flat_map(|node| node.children())
            .filter(|node| node.tag_name().name() == "sheet")
        {
            let name = sheet.attribute("name").unwrap_or_default();
            // `r:id`, in the relationships namespace, Transitional or Strict.
            let id = sheet
                .attributes()
                .find(|attribute| attribute.name() == "id" && attribute.namespace().is_some())
                .map(|attribute| attribute.value())
                .ok_or_else(|| {
                    refused(format!(
                        "the sheet {:?} names no relationship",
                        shortened(name)
                    ))
                })?;
            let rel = rels.iter().find(|rel| rel.id == id).ok_or_else(|| {
                refused(format!(
                    "the sheet {:?}'s relationship {:?} isn't in the workbook's",
                    shortened(name),
                    shortened(id)
                ))
            })?;
            if rel.external {
                return Err(refused(format!(
                    "the sheet {:?} lies outside the workbook",
                    shortened(name)
                )));
            }
            // A chart sheet, dialog sheet or macro sheet holds no cells.
            if rel.kind == "worksheet" {
                sheets.push((name.to_owned(), rel.target.clone()));
            }
        }
        let internal = |kind: &str| {
            rels.iter()
                .find(|rel| rel.kind == kind && !rel.external)
                .map(|rel| rel.target.as_str())
        };
        let strings = match internal("sharedStrings") {
            Some(target) => shared_strings(&package.text(target)?, target)?,
            None => Vec::new(),
        };
        let styles = match internal("styles") {
            Some(target) => match package.small(target)? {
                Some(xml) => date_styles(&tree(&xml, target)?),
                None => Vec::new(),
            },
            None => Vec::new(),
        };
        Ok(Self {
            sheets,
            strings,
            styles,
            date1904,
        })
    }
}

/// Each cell format's date kind: its number format's (`numFmtId`), a custom code from
/// `numFmts` (§18.8.31) or a built-in one (§18.8.30).
fn date_styles(doc: &roxmltree::Document<'_>) -> Vec<Option<DateKind>> {
    let root = doc.root_element();
    let custom: HashMap<&str, &str> = root
        .children()
        .filter(|node| node.tag_name().name() == "numFmts")
        .flat_map(|node| node.children())
        .filter(|node| node.tag_name().name() == "numFmt")
        .filter_map(|node| Some((node.attribute("numFmtId")?, node.attribute("formatCode")?)))
        .collect();
    root.children()
        .filter(|node| node.tag_name().name() == "cellXfs")
        .flat_map(|node| node.children())
        .filter(|node| node.tag_name().name() == "xf")
        .map(|xf| {
            let id = xf.attribute("numFmtId").unwrap_or("0");
            match custom.get(id) {
                Some(code) => format_kind(code),
                None => id
                    .parse::<u32>()
                    .ok()
                    .and_then(|id| BUILT_IN_DATES.iter().find(|(built_in, _)| *built_in == id))
                    .and_then(|(_, code)| format_kind(code)),
            }
        })
        .collect()
}

/// The built-in number formats that show a date or a time, by id, with their codes (§18.8.30).
/// Ids 27 to 36 and 50 to 58 are East Asian dates whose codes the standard leaves to the locale;
/// a number under one is kept as stored.
const BUILT_IN_DATES: [(u32, &str); 12] = [
    (14, "mm-dd-yy"),
    (15, "d-mmm-yy"),
    (16, "d-mmm"),
    (17, "mmm-yy"),
    (18, "h:mm AM/PM"),
    (19, "h:mm:ss AM/PM"),
    (20, "h:mm"),
    (21, "h:mm:ss"),
    (22, "m/d/yy h:mm"),
    (45, "mm:ss"),
    (46, "[h]:mm:ss"),
    (47, "mmss.0"),
];

/// What a number format shows of a serial number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DateKind {
    /// A date, `2023-03-15`.
    Date,
    /// A time of day, `18:00:00`.
    Time,
    /// Both, `2023-03-15 18:00:00`.
    DateTime,
    /// A duration counted in hours past 24, `36:00:00` (`[h]`, `[m]` or `[s]`).
    Elapsed,
}

/// What a number format code shows, from its first section (§18.8.31): `None` for a number.
fn format_kind(code: &str) -> Option<DateKind> {
    let chars: Vec<char> = code.chars().collect();
    // The date and time letters, lowercased, each run of one letter once.
    let mut letters: Vec<char> = Vec::new();
    let mut elapsed = false;
    let push = |letter: char, letters: &mut Vec<char>| {
        if letters.last() != Some(&letter) {
            letters.push(letter);
        }
    };
    let mut at = 0;
    while let Some(&c) = chars.get(at) {
        at += 1;
        match c {
            ';' => break,
            '"' => {
                while chars.get(at).is_some_and(|&c| c != '"') {
                    at += 1;
                }
                at += 1;
            }
            // An escaped character, a space as wide as one, or a fill: the next is literal.
            '\\' | '_' | '*' => at += 1,
            '[' => {
                let start = at;
                while chars.get(at).is_some_and(|&c| c != ']') {
                    at += 1;
                }
                let inner: String = chars[start..at.min(chars.len())]
                    .iter()
                    .map(char::to_ascii_lowercase)
                    .collect();
                at += 1;
                // `[h]`, `[mm]`, `[ss]`: an elapsed time. Colors, conditions and locales aren't.
                if let Some(first) = inner.chars().next()
                    && matches!(first, 'h' | 'm' | 's')
                    && inner.chars().all(|c| c == first)
                {
                    elapsed = true;
                    push(first, &mut letters);
                }
            }
            _ => {
                let lower = c.to_ascii_lowercase();
                let rest: String = chars[at - 1..]
                    .iter()
                    .take(5)
                    .map(char::to_ascii_lowercase)
                    .collect();
                if rest.starts_with("am/pm") {
                    at += 4;
                } else if rest.starts_with("a/p") {
                    at += 2;
                } else if matches!(lower, 'y' | 'd' | 'h' | 'm' | 's') {
                    push(lower, &mut letters);
                }
            }
        }
    }
    let mut date = false;
    let mut time = false;
    for (index, &letter) in letters.iter().enumerate() {
        match letter {
            'y' | 'd' => date = true,
            'h' | 's' => time = true,
            _ => {
                let minute = (index > 0 && letters[index - 1] == 'h')
                    || letters.get(index + 1) == Some(&'s');
                if minute {
                    time = true;
                } else {
                    date = true;
                }
            }
        }
    }
    match (date, time) {
        (true, true) => Some(DateKind::DateTime),
        (true, false) => Some(DateKind::Date),
        (false, true) if elapsed => Some(DateKind::Elapsed),
        (false, true) => Some(DateKind::Time),
        (false, false) => None,
    }
}

/// Days from 1970-01-01 to 1899-12-30, the day before serial 1 in Excel's 1900 date system once
/// past its 1900-02-29.
const EPOCH_1900: i64 = -25_569;
/// Days from 1970-01-01 to 1904-01-01, serial 0 in the 1904 date system.
const EPOCH_1904: i64 = -24_107;
/// Serial 61, 1900-03-01: below it the 1900 system's days are off by its 1900-02-29.
const FIRST_SERIAL_1900: i64 = 61;
/// Serial 2,958,465, 9999-12-31, the last day Excel shows.
const LAST_SERIAL: i64 = 2_958_465;
const MS_A_DAY: i64 = 86_400_000;

/// A serial number written as its format shows it; `None` where the date would be before
/// 1900-03-01 in the 1900 system, before the epoch, or past 9999-12-31.
fn date_text(serial: f64, kind: DateKind, date1904: bool) -> Option<String> {
    if !serial.is_finite() || serial < 0.0 || serial >= (LAST_SERIAL + 1) as f64 {
        return None;
    }
    #[allow(
        clippy::cast_possible_truncation,
        reason = "in range: under 2.6e14 ms, exact in an f64 and an i64"
    )]
    let ms = (serial * MS_A_DAY as f64).round() as i64;
    let days = ms.div_euclid(MS_A_DAY);
    let of_day = ms.rem_euclid(MS_A_DAY);
    let date = || {
        if !date1904 && days < FIRST_SERIAL_1900 {
            return None;
        }
        let epoch = if date1904 { EPOCH_1904 } else { EPOCH_1900 };
        let (year, month, day) = civil_from_days(epoch + days);
        Some(format!("{year:04}-{month:02}-{day:02}"))
    };
    match kind {
        DateKind::Elapsed => Some(clock(ms)),
        DateKind::Time if days == 0 => Some(clock(of_day)),
        DateKind::Date if of_day == 0 => date(),
        // A time of day that also holds a date, or a date that holds a time, keeps both.
        DateKind::Time | DateKind::Date | DateKind::DateTime => {
            Some(format!("{} {}", date()?, clock(of_day)))
        }
    }
}

/// Milliseconds as `H:MM:SS`, hours past 24 counted on, the seconds' fraction written to the
/// millisecond only when it isn't zero, trailing zeros dropped.
fn clock(ms: i64) -> String {
    let hours = ms / 3_600_000;
    let minutes = ms / 60_000 % 60;
    let seconds = ms / 1000 % 60;
    let fraction = ms % 1000;
    let mut text = format!("{hours:02}:{minutes:02}:{seconds:02}");
    if fraction != 0 {
        text.push_str(format!(".{fraction:03}").trim_end_matches('0'));
    }
    text
}

/// The proleptic Gregorian date `days` after 1970-01-01: Howard Hinnant's `civil_from_days`,
/// from *chrono-Compatible Low-Level Date Algorithms* (public domain).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a day of a month and a month of a year"
    )]
    (
        yoe + era * 400 + i64::from(month <= 2),
        month as u32,
        day as u32,
    )
}

/// A streaming reader's events, every refusal named for its part.
struct Events<'a> {
    reader: quick_xml::Reader<&'a [u8]>,
    part: &'a str,
}

impl<'a> Events<'a> {
    fn new(xml: &'a str, part: &'a str) -> Self {
        Self {
            reader: quick_xml::Reader::from_str(xml),
            part,
        }
    }

    fn next(&mut self) -> Result<Event<'a>, LogError> {
        let event = self.reader.read_event().map_err(|error| {
            refused(format!(
                "the part {:?} isn't well-formed XML at byte {}: {error}",
                shortened(self.part),
                self.reader.error_position()
            ))
        })?;
        if matches!(event, Event::DocType(_)) {
            return Err(refused(format!(
                "the part {:?} holds a document type declaration",
                shortened(self.part)
            )));
        }
        Ok(event)
    }

    /// An event's text, XML's own references resolved; `None` for an event that holds none.
    fn text(&self, event: &Event<'_>) -> Result<Option<String>, LogError> {
        Ok(match event {
            Event::Text(text) => Some(text.xml10_content().into_owned()),
            Event::CData(data) => Some(data.xml10_content().into_owned()),
            Event::GeneralRef(reference) => {
                let resolved = if reference.is_char_ref() {
                    reference
                        .resolve_char_ref()
                        .ok()
                        .flatten()
                        .map(String::from)
                } else {
                    quick_xml::escape::resolve_predefined_entity(&reference.xml10_content())
                        .map(str::to_owned)
                };
                Some(resolved.ok_or_else(|| {
                    refused(format!(
                        "the part {:?} refers to an entity that isn't XML's own",
                        shortened(self.part)
                    ))
                })?)
            }
            _ => None,
        })
    }

    /// An element's attribute by its local name, normalized.
    fn attribute(&self, element: &BytesStart<'_>, name: &str) -> Result<Option<String>, LogError> {
        for attribute in element.attributes() {
            let attribute = attribute.map_err(|error| {
                refused(format!(
                    "an attribute in the part {:?} isn't well-formed: {error}",
                    shortened(self.part)
                ))
            })?;
            if attribute.key.local_name().as_ref() == name {
                let value = attribute
                    .normalized_value(XmlVersion::Implicit1_0)
                    .map_err(|error| {
                        refused(format!(
                            "an attribute in the part {:?} isn't well-formed: {error}",
                            shortened(self.part)
                        ))
                    })?;
                return Ok(Some(value.into_owned()));
            }
        }
        Ok(None)
    }
}

/// The shared strings (`sst`, §18.4.9), each `si`'s text: its `t`, or every run's `t`, a
/// phonetic guide's (`rPh`) left out.
fn shared_strings(xml: &str, part: &str) -> Result<Vec<String>, LogError> {
    let mut events = Events::new(xml, part);
    let mut strings = Vec::new();
    let mut current: Option<String> = None;
    let mut in_text = false;
    let mut phonetic = 0usize;
    loop {
        let event = events.next()?;
        match &event {
            Event::Start(element) => match element.local_name().as_ref() {
                "si" => current = Some(String::new()),
                "rPh" => phonetic += 1,
                "t" => in_text = phonetic == 0 && current.is_some(),
                _ => {}
            },
            Event::Empty(element) if element.local_name().as_ref() == "si" => {
                strings.push(String::new());
            }
            Event::End(element) => match element.local_name().as_ref() {
                "si" => strings.extend(current.take()),
                "rPh" => phonetic = phonetic.saturating_sub(1),
                "t" => in_text = false,
                _ => {}
            },
            Event::Eof => break,
            _ => {
                if in_text
                    && let Some(text) = events.text(&event)?
                    && let Some(current) = current.as_mut()
                {
                    current.push_str(&text);
                }
            }
        }
    }
    Ok(strings)
}

/// A cell being read: where it is, its type and style, and its value's text so far.
struct Cell {
    row: u32,
    column: u32,
    kind: Option<String>,
    style: Option<usize>,
    /// Whether it holds a formula (`f`), whose value is the one saved with it.
    formula: bool,
    value: Option<String>,
    inline: Option<String>,
}

/// What a sheet's text leaves out, counted.
#[derive(Default)]
struct Counts {
    errors: usize,
    unsaved: usize,
    dates_kept: usize,
}

impl Counts {
    fn notes(&self) -> Vec<String> {
        let mut notes = Vec::new();
        if self.errors > 0 {
            notes.push(format!(
                "{} cells holding an error, such as `#DIV/0!`, written as gaps",
                self.errors
            ));
        }
        if self.unsaved > 0 {
            notes.push(format!(
                "{} formula cells with no value saved with them written as gaps",
                self.unsaved
            ));
        }
        if self.dates_kept > 0 {
            notes.push(format!(
                "{} numbers under a date format kept as stored: their dates would fall before \
                 1900-03-01 or past 9999-12-31",
                self.dates_kept
            ));
        }
        notes
    }
}

/// A worksheet's text as it is written: each row's cells, its separators and gaps, as they come;
/// each row's trailing padding waits for the sheet's width, known only at its end.
struct Lines<'b> {
    body: String,
    /// Each written row's end in `body` and its last column.
    ends: Vec<(usize, u32)>,
    /// The row being written: where it starts in `body`, and the column after its last cell.
    start: usize,
    next: u32,
    budget: &'b mut Budget,
}

impl Lines<'_> {
    /// Starts a row.
    fn open(&mut self) {
        self.start = self.body.len();
        self.next = 1;
    }

    /// Writes a cell's text in its column, after the commas before it, gaps included.
    fn cell(&mut self, column: u32, value: &str) -> Result<(), LogError> {
        let before = self.body.len();
        // Columns ascend, checked as each cell is placed.
        let separators = (column - self.next) as usize + usize::from(self.next > 1);
        self.budget.take(separators)?;
        for _ in 0..separators {
            self.body.push(',');
        }
        push_cell(&mut self.body, value);
        // The value's own bytes were counted as it was read: only its quotes are new.
        let written = self.body.len() - before;
        self.budget
            .take(written.saturating_sub(separators + value.len()))?;
        self.next = column + 1;
        Ok(())
    }

    /// Ends a row: kept if it holds a value, else taken back.
    fn close(&mut self) {
        if self.next > 1 {
            self.ends.push((self.body.len(), self.next - 1));
        } else {
            self.body.truncate(self.start);
        }
        self.next = 1;
    }

    /// The sheet's text: every row as wide as the widest, each ending in `\n`.
    fn finish(self) -> Result<String, LogError> {
        let width = self.ends.iter().map(|(_, last)| *last).max().unwrap_or(0);
        let mut text = String::new();
        let mut from = 0;
        for (end, last) in &self.ends {
            let padding = (width - last) as usize;
            self.budget.take(padding + 1)?;
            text.push_str(&self.body[from..*end]);
            for _ in 0..padding {
                text.push(',');
            }
            text.push('\n');
            from = *end;
        }
        Ok(text)
    }
}

/// A worksheet's cells (`sheetData`, §18.3.1.80) as text, one line a row with a value, and the
/// notes of what the text leaves out.
fn read_sheet(
    xml: &str,
    part: &str,
    book: &Book,
    budget: &mut Budget,
) -> Result<(String, Vec<String>), LogError> {
    let mut events = Events::new(xml, part);
    let mut lines = Lines {
        body: String::new(),
        ends: Vec::new(),
        start: 0,
        next: 1,
        budget,
    };
    // The row being read, by its number.
    let mut row: Option<u32> = None;
    let mut last_row = 0u32;
    let mut last_column = 0u32;
    let mut cell: Option<Cell> = None;
    let mut in_data = false;
    let mut in_value = false;
    let mut in_inline = false;
    let mut phonetic = 0usize;
    let mut counts = Counts::default();
    loop {
        let event = events.next()?;
        match &event {
            Event::Start(element) | Event::Empty(element) => {
                let empty = matches!(event, Event::Empty(_));
                match element.local_name().as_ref() {
                    "sheetData" => in_data = !empty,
                    "row" if in_data && row.is_none() => {
                        let number = match events.attribute(element, "r")? {
                            Some(r) => {
                                r.parse::<u32>().ok().filter(|r| *r >= 1).ok_or_else(|| {
                                    refused(format!(
                                        "a row in {part:?} has the number {:?}",
                                        shortened(&r)
                                    ))
                                })?
                            }
                            None => last_row + 1,
                        };
                        if number <= last_row || number > MAX_ROWS {
                            return Err(refused(format!(
                                "row {number} in {part:?} comes after row {last_row}, or past \
                                 row {MAX_ROWS}"
                            )));
                        }
                        last_row = number;
                        last_column = 0;
                        if !empty {
                            row = Some(number);
                            lines.open();
                        }
                    }
                    "c" if in_data => {
                        let (Some(number), None) = (row, cell.as_ref()) else {
                            return Err(refused(format!(
                                "a cell in {part:?} lies outside a row, or inside another cell"
                            )));
                        };
                        let column = match events.attribute(element, "r")? {
                            Some(reference) => {
                                let (column, at_row) =
                                    cell_reference(&reference).ok_or_else(|| {
                                        refused(format!(
                                            "a cell in {part:?} has the reference {:?}",
                                            shortened(&reference)
                                        ))
                                    })?;
                                if at_row != number {
                                    return Err(refused(format!(
                                        "the cell {:?} in {part:?} is in row {number}",
                                        shortened(&reference)
                                    )));
                                }
                                column
                            }
                            None => last_column + 1,
                        };
                        if column <= last_column || column > MAX_COLUMNS {
                            return Err(refused(format!(
                                "a cell in row {number} of {part:?} is in column {column}, \
                                 after column {last_column}, or past column {MAX_COLUMNS}"
                            )));
                        }
                        last_column = column;
                        let style = events
                            .attribute(element, "s")?
                            .map(|s| {
                                s.parse::<usize>().map_err(|_| {
                                    refused(format!(
                                        "a cell in row {number} of {part:?} has the style {:?}",
                                        shortened(&s)
                                    ))
                                })
                            })
                            .transpose()?;
                        let new = Cell {
                            row: number,
                            column,
                            kind: events.attribute(element, "t")?,
                            style,
                            formula: false,
                            value: None,
                            inline: None,
                        };
                        if empty {
                            finish_cell(new, book, &mut lines, &mut counts, part)?;
                        } else {
                            cell = Some(new);
                        }
                    }
                    "f" => {
                        if let Some(cell) = cell.as_mut() {
                            cell.formula = true;
                        }
                    }
                    "v" if !empty => {
                        if let Some(cell) = cell.as_mut() {
                            in_value = true;
                            cell.value.get_or_insert_with(String::new);
                        }
                    }
                    "is" if !empty => {
                        if let Some(cell) = cell.as_mut() {
                            cell.inline.get_or_insert_with(String::new);
                        }
                    }
                    "rPh" if !empty => phonetic += 1,
                    "t" if !empty => {
                        in_inline = phonetic == 0
                            && cell.as_ref().is_some_and(|cell| cell.inline.is_some());
                    }
                    _ => {}
                }
            }
            Event::End(element) => match element.local_name().as_ref() {
                "sheetData" => in_data = false,
                "row" if row.is_some() && cell.is_none() => {
                    row = None;
                    lines.close();
                }
                "c" => {
                    if let Some(done) = cell.take() {
                        finish_cell(done, book, &mut lines, &mut counts, part)?;
                    }
                }
                "v" => in_value = false,
                "t" => in_inline = false,
                "rPh" => phonetic = phonetic.saturating_sub(1),
                _ => {}
            },
            Event::Eof => break,
            _ => {
                if (in_value || in_inline)
                    && let Some(text) = events.text(&event)?
                    && let Some(cell) = cell.as_mut()
                {
                    let target = if in_value {
                        cell.value.get_or_insert_with(String::new)
                    } else {
                        cell.inline.get_or_insert_with(String::new)
                    };
                    lines.budget.take(text.len())?;
                    target.push_str(&text);
                }
            }
        }
    }
    if row.is_some() || cell.is_some() {
        return Err(refused(format!("the part {part:?} ends inside a row")));
    }
    Ok((lines.finish()?, counts.notes()))
}

/// A cell's text as the workbook stores it, by its type (§18.18.11), written in its row; a cell
/// with no value is a gap.
fn finish_cell(
    cell: Cell,
    book: &Book,
    lines: &mut Lines<'_>,
    counts: &mut Counts,
    part: &str,
) -> Result<(), LogError> {
    let at = || {
        format!(
            "the cell in row {}, column {}, of {part:?}",
            cell.row, cell.column
        )
    };
    let value = cell.value.as_deref().map(str::trim);
    let text: Option<Cow<'_, str>> = match cell.kind.as_deref().unwrap_or("n") {
        "n" => match value.filter(|value| !value.is_empty()) {
            None => None,
            Some(value) => {
                let number = value
                    .parse::<f64>()
                    .ok()
                    .filter(|number| number.is_finite())
                    .ok_or_else(|| {
                        refused(format!(
                            "{} holds {:?}, not a number",
                            at(),
                            shortened(value)
                        ))
                    })?;
                let kind = cell
                    .style
                    .and_then(|style| book.styles.get(style).copied().flatten());
                match kind.map(|kind| date_text(number, kind, book.date1904)) {
                    Some(Some(date)) => {
                        lines.budget.take(date.len())?;
                        Some(Cow::Owned(date))
                    }
                    Some(None) => {
                        counts.dates_kept += 1;
                        Some(Cow::Borrowed(value))
                    }
                    None => Some(Cow::Borrowed(value)),
                }
            }
        },
        "s" => match value {
            None => None,
            Some(value) => {
                let string = value
                    .parse::<usize>()
                    .ok()
                    .and_then(|index| book.strings.get(index))
                    .ok_or_else(|| {
                        refused(format!(
                            "{} names the shared string {:?}, which isn't there",
                            at(),
                            shortened(value)
                        ))
                    })?;
                lines.budget.take(string.len())?;
                Some(Cow::Borrowed(string.as_str()))
            }
        },
        "inlineStr" => cell
            .inline
            .as_deref()
            .or(cell.value.as_deref())
            .map(Cow::Borrowed),
        "str" => cell.value.as_deref().map(Cow::Borrowed),
        "d" => value.map(Cow::Borrowed),
        "b" => match value {
            None => None,
            Some("1") => Some(Cow::Borrowed("TRUE")),
            Some("0") => Some(Cow::Borrowed("FALSE")),
            Some(other) => {
                return Err(refused(format!(
                    "{} holds {:?}, not a truth value",
                    at(),
                    shortened(other)
                )));
            }
        },
        "e" => {
            counts.errors += 1;
            return Ok(());
        }
        other => {
            return Err(refused(format!(
                "{} has the type {:?}, which isn't one",
                at(),
                shortened(other)
            )));
        }
    };
    match text {
        Some(text) if !text.is_empty() => lines.cell(cell.column, &text)?,
        _ if cell.formula => counts.unsaved += 1,
        _ => {}
    }
    Ok(())
}

/// A cell reference's column and row, both from 1: `B12` is (2, 12); `None` for anything else.
fn cell_reference(reference: &str) -> Option<(u32, u32)> {
    let split = reference.find(|c: char| c.is_ascii_digit())?;
    let (letters, digits) = reference.split_at(split);
    if letters.is_empty() || letters.len() > 3 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let mut column = 0u32;
    for letter in letters.bytes() {
        if !letter.is_ascii_uppercase() {
            return None;
        }
        column = column * 26 + u32::from(letter - b'A' + 1);
    }
    let row = digits.parse::<u32>().ok().filter(|row| *row >= 1)?;
    Some((column, row))
}

/// A cell's text in a comma-separated line: quoted if it holds a comma or a double quote, its
/// quotes doubled; a line break becomes a space.
fn push_cell(out: &mut String, value: &str) {
    let value: Cow<'_, str> = if value.contains(['\r', '\n']) {
        Cow::Owned(value.replace("\r\n", " ").replace(['\r', '\n'], " "))
    } else {
        Cow::Borrowed(value)
    };
    if value.contains([',', '"']) {
        out.push('"');
        out.push_str(&value.replace('"', "\"\""));
        out.push('"');
    } else {
        out.push_str(&value);
    }
}

/// A name cut to 60 characters for an error, so a hostile part's long name isn't copied whole.
fn shortened(name: &str) -> String {
    const MOST: usize = 60;
    if name.chars().count() <= MOST {
        name.to_owned()
    } else {
        name.chars().take(MOST).collect::<String>() + "…"
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Write};

    use proptest::prelude::*;
    use zip::write::SimpleFileOptions;

    use super::*;
    use crate::log::LogFormat;

    const RELS: &str = r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="r1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#;
    const MAIN: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
    const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    const TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

    /// An archive of the given parts, each deflated.
    fn zipped(parts: &[(&str, &str)]) -> Vec<u8> {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for (name, text) in parts {
            zip.start_file(*name, SimpleFileOptions::default()).unwrap();
            zip.write_all(text.as_bytes()).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    /// A workbook of the given sheets' `sheetData` contents, with shared strings and the styles'
    /// `numFmts` and `cellXfs` contents when given, and `workbookPr` attributes.
    struct Book<'a> {
        sheets: Vec<(&'a str, String)>,
        strings: Option<&'a str>,
        styles: Option<&'a str>,
        properties: &'a str,
    }

    impl<'a> Book<'a> {
        fn one(data: &str) -> Self {
            Book {
                sheets: vec![("Flight", data.to_owned())],
                strings: None,
                styles: None,
                properties: "",
            }
        }

        fn bytes(&self) -> Vec<u8> {
            let mut list = String::new();
            let mut rels = String::new();
            let mut parts: Vec<(String, String)> = Vec::new();
            for (n, (name, data)) in self.sheets.iter().enumerate() {
                list += &format!(r#"<sheet name="{name}" sheetId="{}" r:id="s{n}"/>"#, n + 1);
                rels += &format!(
                    r#"<Relationship Id="s{n}" Type="{TYPE}/worksheet" Target="worksheets/sheet{n}.xml"/>"#
                );
                parts.push((
                    format!("xl/worksheets/sheet{n}.xml"),
                    format!(
                        r#"<worksheet xmlns="{MAIN}"><sheetData>{data}</sheetData></worksheet>"#
                    ),
                ));
            }
            if let Some(strings) = self.strings {
                rels += &format!(
                    r#"<Relationship Id="t" Type="{TYPE}/sharedStrings" Target="sharedStrings.xml"/>"#
                );
                parts.push((
                    "xl/sharedStrings.xml".to_owned(),
                    format!(r#"<sst xmlns="{MAIN}">{strings}</sst>"#),
                ));
            }
            if let Some(styles) = self.styles {
                rels +=
                    &format!(r#"<Relationship Id="y" Type="{TYPE}/styles" Target="styles.xml"/>"#);
                parts.push((
                    "xl/styles.xml".to_owned(),
                    format!(r#"<styleSheet xmlns="{MAIN}">{styles}</styleSheet>"#),
                ));
            }
            parts.push((
                "xl/workbook.xml".to_owned(),
                format!(
                    r#"<workbook xmlns="{MAIN}" xmlns:r="{R}"><workbookPr {}/><sheets>{list}</sheets></workbook>"#,
                    self.properties
                ),
            ));
            parts.push((
                "xl/_rels/workbook.xml.rels".to_owned(),
                format!(
                    r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">{rels}</Relationships>"#
                ),
            ));
            parts.push(("_rels/.rels".to_owned(), RELS.to_owned()));
            let refs: Vec<(&str, &str)> = parts
                .iter()
                .map(|(name, text)| (name.as_str(), text.as_str()))
                .collect();
            zipped(&refs)
        }
    }

    fn text_of(book: &Book<'_>) -> String {
        sheets(&book.bytes()).unwrap().remove(0).text
    }

    fn refusal(book: &Book<'_>) -> String {
        match sheets(&book.bytes()) {
            Err(LogError::Workbook { message }) => message,
            other => panic!("expected a workbook refusal, got {other:?}"),
        }
    }

    #[test]
    fn each_kind_of_cell_is_written_as_stored() {
        let book = Book {
            strings: Some(
                // 0: plain; 1: two runs and a phonetic guide; 2: a comma and a quote; 3: two lines.
                r#"<si><t>Time (s)</t></si><si><r><t>Alt</t></r><r><t>itude (m)</t></r><rPh sb="0" eb="1"><t>X</t></rPh></si><si><t>a, "b"</t></si><si><t>line
two</t></si>"#,
            ),
            ..Book::one(
                r#"<row r="1"><c r="A1" t="s"><v>0</v></c><c r="B1" t="s"><v>1</v></c></row>
                <row r="2"><c r="A2"><v>0.10000000000000001</v></c><c r="B2" t="inlineStr"><is><t>x &amp; &#x41;</t></is></c><c r="C2" t="str"><f>A2</f><v>made</v></c></row>
                <row r="3"><c r="A3" t="e"><v>#DIV/0!</v></c><c r="B3" t="b"><v>1</v></c><c r="C3" t="b"><v>0</v></c><c r="D3" t="s"><v>2</v></c></row>
                <row r="4"><c r="A4"><f>B4</f></c><c r="B4" t="d"><v>2023-03-15T18:00:00</v></c><c r="C4" t="s"><v>3</v></c></row>"#,
            )
        };
        let sheet = sheets(&book.bytes()).unwrap().remove(0);
        assert_eq!(
            sheet.text,
            "Time (s),Altitude (m),,\n\
             0.10000000000000001,x & A,made,\n\
             ,TRUE,FALSE,\"a, \"\"b\"\"\"\n\
             ,2023-03-15T18:00:00,line two,\n"
        );
        assert_eq!(sheet.notes.len(), 2, "{:?}", sheet.notes);
        assert!(
            sheet.notes[0].starts_with("1 cells holding an error"),
            "{:?}",
            sheet.notes
        );
        assert!(
            sheet.notes[1].starts_with("1 formula cells"),
            "{:?}",
            sheet.notes
        );
    }

    #[test]
    fn gaps_are_kept_and_rows_padded_to_the_widest() {
        // A gap in column B, a short row, a formatted cell with no value past the last value (it
        // widens nothing), an empty row and a row with no `r` (the next number).
        let text = text_of(&Book::one(
            r#"<row r="1"><c r="A1"><v>1</v></c><c r="C1"><v>3</v></c><c r="F1" s="0"/></row>
            <row r="2"><c r="A2"><v>4</v></c></row>
            <row r="3"></row><row r="5"/>
            <row><c><v>7</v></c><c><v>8</v></c></row>"#,
        ));
        assert_eq!(text, "1,,3\n4,,\n7,8,\n");
    }

    #[test]
    fn a_sheet_reads_as_the_file_it_came_from() {
        let book = Book {
            sheets: vec![
                (
                    "Data",
                    r#"<row r="1"><c r="A1" t="inlineStr"><is><t>Time</t></is></c><c r="B1" t="inlineStr"><is><t>Altitude</t></is></c><c r="C1" t="inlineStr"><is><t>Velocity</t></is></c></row>
                    <row r="2"><c r="A2"><v>0</v></c><c r="B2"><v>0</v></c><c r="C2"><v>0</v></c></row>
                    <row r="3"><c r="A3"><v>0.05</v></c><c r="B3"><v>1</v></c><c r="C3"><v>8</v></c></row>"#
                        .to_owned(),
                ),
                ("Chart", String::new()),
            ],
            ..Book::one("")
        };
        let logs = read(&book.bytes()).unwrap();
        assert_eq!(logs.len(), 2);
        let log = logs[0].log.as_ref().unwrap();
        assert_eq!(log.format, LogFormat::PerfectFliteCsv);
        assert!(log.notes[0].contains("sheet \"Data\""), "{:?}", log.notes);
        assert_eq!(logs[1].name, "Chart");
        assert_eq!(logs[1].log, Err(LogError::NoData { format: FORMAT }));
    }

    #[test]
    fn a_number_under_a_date_format_is_its_date() {
        // Serial 45,000 is 2023-03-15: 45,000 − 25,569 = 19,431 days after 1970-01-01, and
        // 2023-01-01 is 19,358 (53 years, 13 of them leap); 73 days on is 14 March's day after.
        let styles = r#"<numFmts count="4"><numFmt numFmtId="164" formatCode="yyyy\-mm\-dd\ hh:mm:ss.000"/><numFmt numFmtId="165" formatCode="0.00&quot;m&quot;"/><numFmt numFmtId="166" formatCode="[Red]0.0"/><numFmt numFmtId="167" formatCode="mm:ss.0;@"/></numFmts>
            <cellXfs count="10"><xf numFmtId="0"/><xf numFmtId="14"/><xf numFmtId="21"/><xf numFmtId="22"/><xf numFmtId="46"/><xf numFmtId="164"/><xf numFmtId="165"/><xf numFmtId="166"/><xf numFmtId="167"/><xf numFmtId="11"/></cellXfs>"#;
        let book = Book {
            styles: Some(styles),
            ..Book::one(
                r#"<row r="1"><c r="A1" s="1"><v>45000</v></c><c r="B1" s="2"><v>0.5</v></c><c r="C1" s="3"><v>45000.75</v></c><c r="D1" s="4"><v>1.5</v></c><c r="E1" s="5"><v>45000.0000434028</v></c></row>
                <row r="2"><c r="A2" s="6"><v>45000</v></c><c r="B2" s="7"><v>45000</v></c><c r="C2" s="8"><v>0.0006</v></c><c r="D2" s="9"><v>45000</v></c><c r="E2" s="0"><v>45000</v></c></row>
                <row r="3"><c r="A3" s="1"><v>59</v></c><c r="B3" s="1"><v>2958466</v></c><c r="C3" s="1"><v>45000.25</v></c><c r="D3" s="2"><v>1.25</v></c><c r="E3" s="12"><v>3</v></c></row>"#,
            )
        };
        let sheet = sheets(&book.bytes()).unwrap().remove(0);
        // E1: 0.0000434028 days is 3.750002 s, 3.75 to the millisecond. C2: 0.0006 days is 51.84 s.
        // E3's style isn't in the list: a number. A3 (1900-02-28), B3 (past 9999-12-31) and D3, a
        // time of day that also holds serial 1's day, 1900-01-01, are kept as stored.
        assert_eq!(
            sheet.text,
            "2023-03-15,12:00:00,2023-03-15 18:00:00,36:00:00,2023-03-15 00:00:03.75\n\
             45000,45000,00:00:51.84,45000,45000\n\
             59,2958466,2023-03-15 06:00:00,1.25,3\n"
        );
        assert_eq!(sheet.notes.len(), 1, "{:?}", sheet.notes);
        assert!(
            sheet.notes[0].starts_with("3 numbers under a date format kept"),
            "{:?}",
            sheet.notes
        );
    }

    #[test]
    fn the_1904_date_system_counts_from_1904() {
        let book = Book {
            styles: Some(r#"<cellXfs><xf numFmtId="0"/><xf numFmtId="14"/></cellXfs>"#),
            properties: r#"date1904="1""#,
            ..Book::one(
                r#"<row r="1"><c r="A1" s="1"><v>0</v></c><c r="B1" s="1"><v>43538</v></c></row>"#,
            )
        };
        // 43,538 = 45,000 − 1,462, the days between the two systems' epochs.
        assert_eq!(text_of(&book), "1904-01-01,2023-03-15\n");
    }

    #[test]
    fn format_codes_are_told_apart() {
        for (code, kind) in [
            ("General", None),
            ("0.00E+00", None),
            ("#,##0.00", None),
            ("@", None),
            ("0.0 \"ms\"", None),
            ("0.0\\s", None),
            ("[Blue]0.00", None),
            ("[$-409]0.00", None),
            ("mm-dd-yy", Some(DateKind::Date)),
            ("d-mmm", Some(DateKind::Date)),
            ("mmm-yy", Some(DateKind::Date)),
            ("h:mm AM/PM", Some(DateKind::Time)),
            ("hh:mm:ss", Some(DateKind::Time)),
            ("mm:ss", Some(DateKind::Time)),
            ("mmss.0", Some(DateKind::Time)),
            ("m/d/yy h:mm", Some(DateKind::DateTime)),
            ("yyyy-mm-dd\\ hh:mm:ss", Some(DateKind::DateTime)),
            ("[h]:mm:ss", Some(DateKind::Elapsed)),
            ("[mm]:ss", Some(DateKind::Elapsed)),
            ("[$-F800]dddd, mmmm dd, yyyy", Some(DateKind::Date)),
            ("0.00;hh:mm", None),
            ("\"d\"0", None),
            ("a/p h", Some(DateKind::Time)),
        ] {
            assert_eq!(format_kind(code), kind, "{code}");
        }
    }

    #[test]
    fn clocks_and_calendars() {
        assert_eq!(clock(0), "00:00:00");
        assert_eq!(clock(3_723_500), "01:02:03.5");
        assert_eq!(clock(90_000_001), "25:00:00.001");
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(-25_569 + 61), (1900, 3, 1));
        assert_eq!(civil_from_days(11_016), (2000, 2, 29));
        // The last day: serial 2,958,465.
        assert_eq!(civil_from_days(EPOCH_1900 + LAST_SERIAL), (9999, 12, 31));
        // A time of day of 23:59:59.9996 rounds into the next day.
        assert_eq!(
            date_text(
                45_000.0 + 86_399.999_6 / 86_400.0,
                DateKind::DateTime,
                false
            )
            .as_deref(),
            Some("2023-03-16 00:00:00")
        );
    }

    #[test]
    fn cells_and_rows_out_of_order_are_refused() {
        for (data, words) in [
            (
                r#"<row r="1"><c r="B1"><v>1</v></c><c r="A1"><v>2</v></c></row>"#,
                "after column 2",
            ),
            (
                r#"<row r="1"><c r="A1"><v>1</v></c><c r="A1"><v>2</v></c></row>"#,
                "after column 1",
            ),
            (r#"<row r="2"/><row r="1"/>"#, "comes after row 2"),
            (r#"<row r="1"><c r="A2"><v>1</v></c></row>"#, "is in row 1"),
            (
                r#"<row r="1"><c r="XFE1"><v>1</v></c></row>"#,
                "past column 16384",
            ),
            (r#"<row r="1048577"/>"#, "past row 1048576"),
            (
                r#"<row r="1"><c r="a1"><v>1</v></c></row>"#,
                "has the reference",
            ),
            (
                r#"<row r="1"><c r="A1"><v>1e999</v></c></row>"#,
                "not a number",
            ),
            (r#"<row r="1"><c r="A1"><v>x</v></c></row>"#, "not a number"),
            (
                r#"<row r="1"><c r="A1" t="s"><v>0</v></c></row>"#,
                "isn't there",
            ),
            (
                r#"<row r="1"><c r="A1" t="b"><v>2</v></c></row>"#,
                "not a truth value",
            ),
            (
                r#"<row r="1"><c r="A1" t="q"><v>2</v></c></row>"#,
                "isn't one",
            ),
            (
                r#"<row r="1"><c r="A1" s="x"><v>2</v></c></row>"#,
                "has the style",
            ),
            (r#"<c r="A1"><v>2</v></c>"#, "outside a row"),
            (
                r#"<row r="1"><c r="A1"><v>&lol;</v></c></row>"#,
                "isn't XML's own",
            ),
        ] {
            let message = refusal(&Book::one(data));
            assert!(message.contains(words), "{data}: {message}");
        }
    }

    #[test]
    fn a_document_type_declaration_is_refused() {
        let doctype = r#"<!DOCTYPE x [<!ENTITY a "aaaa">]>"#;
        let book = Book::one("");
        let mut parts: Vec<(String, String)> = Vec::new();
        let bytes = book.bytes();
        for member in archive::unpack(&bytes).unwrap() {
            let mut text = String::from_utf8(member.bytes).unwrap();
            if member.name.ends_with("sheet0.xml") {
                text = format!("{doctype}{text}");
            }
            parts.push((member.name, text));
        }
        let refs: Vec<(&str, &str)> = parts
            .iter()
            .map(|(n, t)| (n.as_str(), t.as_str()))
            .collect();
        match sheets(&zipped(&refs)) {
            Err(LogError::Workbook { message }) => {
                assert!(message.contains("document type declaration"), "{message}");
            }
            other => panic!("expected a refusal, got {other:?}"),
        }
    }

    #[test]
    fn the_text_is_bounded() {
        // 40 cells of one 1,000-byte shared string: 40,000 bytes of text from 2 KB of XML.
        let long = "x".repeat(1000);
        let strings = format!("<si><t>{long}</t></si>");
        let cells: String = (1..=40)
            .map(|row| format!(r#"<row r="{row}"><c r="A{row}" t="s"><v>0</v></c></row>"#))
            .collect();
        let book = Book {
            strings: Some(&strings),
            ..Book::one(&cells)
        };
        let bytes = book.bytes();
        assert_eq!(sheets_within(&bytes, 41_000).unwrap()[0].text.len(), 40_040);
        match sheets_within(&bytes, 39_000) {
            Err(LogError::Workbook { message }) => assert!(message.contains("39000"), "{message}"),
            other => panic!("expected the bound, got {other:?}"),
        }
        // Gaps count: one cell in column XFD writes 16,383 commas.
        let wide = Book::one(r#"<row r="1"><c r="XFD1"><v>1</v></c></row>"#).bytes();
        assert!(sheets_within(&wide, 16_000).is_err());
        assert_eq!(sheets_within(&wide, 16_400).unwrap()[0].text.len(), 16_385);
    }

    #[test]
    fn what_isnt_a_workbook_is_told() {
        assert!(matches!(
            sheets(b"Time,Altitude\n"),
            Err(LogError::NotThisFormat { .. })
        ));
        // A zip archive with no office document, and one whose document isn't a workbook.
        assert!(matches!(
            sheets(&zipped(&[("a.csv", "Time,Altitude\n")])),
            Err(LogError::NotThisFormat { .. })
        ));
        let docx = zipped(&[
            ("_rels/.rels", RELS),
            (
                "xl/workbook.xml",
                r#"<document xmlns="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#,
            ),
        ]);
        assert!(matches!(sheets(&docx), Err(LogError::NotThisFormat { .. })));
        // A sheet named in the workbook whose part is missing.
        let mut book = Book::one("").bytes();
        let members = archive::unpack(&book).unwrap();
        let parts: Vec<(&str, &str)> = members
            .iter()
            .filter(|m| !m.name.ends_with("sheet0.xml"))
            .map(|m| (m.name.as_str(), std::str::from_utf8(&m.bytes).unwrap()))
            .collect();
        book = zipped(&parts);
        assert!(refusal_of(&book).contains("is missing"));
    }

    fn refusal_of(bytes: &[u8]) -> String {
        match sheets(bytes) {
            Err(LogError::Workbook { message }) => message,
            other => panic!("expected a workbook refusal, got {other:?}"),
        }
    }

    #[test]
    fn two_sheets_may_not_share_a_part() {
        let bytes = Book::one("").bytes();
        let mut parts: Vec<(String, String)> = Vec::new();
        for member in archive::unpack(&bytes).unwrap() {
            let mut text = String::from_utf8(member.bytes).unwrap();
            if member.name == "xl/workbook.xml" {
                text = text.replace(
                    "</sheets>",
                    r#"<sheet name="Again" sheetId="2" r:id="s0"/></sheets>"#,
                );
            }
            parts.push((member.name, text));
        }
        let refs: Vec<(&str, &str)> = parts
            .iter()
            .map(|(n, t)| (n.as_str(), t.as_str()))
            .collect();
        assert!(refusal_of(&zipped(&refs)).contains("share the part"));
    }

    #[test]
    fn prefixed_elements_and_strict_namespaces_read() {
        let strict = "http://purl.oclc.org/ooxml/spreadsheetml/main";
        let parts = [
            ("_rels/.rels", RELS.to_owned()),
            (
                "xl/workbook.xml",
                format!(
                    r#"<x:workbook xmlns:x="{strict}" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships"><x:sheets><x:sheet name="S" sheetId="1" r:id="a"/></x:sheets></x:workbook>"#
                ),
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="a" Type="http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet" Target="/xl/worksheets/../sheets/one.xml"/></Relationships>"#.to_owned(),
            ),
            (
                "xl/sheets/one.xml",
                format!(
                    r#"<x:worksheet xmlns:x="{strict}"><x:sheetData><x:row x:r="1"><x:c r="B1"><x:v>2</x:v></x:c></x:row></x:sheetData></x:worksheet>"#
                ),
            ),
        ];
        let refs: Vec<(&str, &str)> = parts.iter().map(|(n, t)| (*n, t.as_str())).collect();
        assert_eq!(sheets(&zipped(&refs)).unwrap()[0].text, ",2\n");
    }

    #[test]
    fn a_workbook_given_to_import_is_sent_to_this_module() {
        match import::read(&Book::one("").bytes()) {
            Err(LogError::Archive { message }) => {
                assert!(message.contains("workbook::read"), "{message}");
            }
            other => panic!("expected the archive refusal, got {other:?}"),
        }
    }

    proptest! {
        /// Any cells, in order, come back in their columns, the commas and quotes undone.
        #[test]
        fn cells_come_back_in_their_columns(
            rows in proptest::collection::vec(
                proptest::collection::btree_map(1u32..12, "[a-z ,\"]{1,6}", 1..5),
                1..6,
            )
        ) {
            let mut data = String::new();
            for (n, cells) in rows.iter().enumerate() {
                let r = n + 1;
                data += &format!(r#"<row r="{r}">"#);
                for (column, text) in cells {
                    let letter = char::from(b'A' + u8::try_from(column - 1).unwrap());
                    let escaped = text.replace('&', "&amp;").replace('"', "&quot;");
                    data += &format!(
                        r#"<c r="{letter}{r}" t="inlineStr"><is><t xml:space="preserve">{escaped}</t></is></c>"#
                    );
                }
                data += "</row>";
            }
            let text = text_of(&Book::one(&data));
            let width = rows.iter().filter_map(|cells| cells.keys().last()).max().copied().unwrap();
            let lines: Vec<&str> = text.lines().collect();
            prop_assert_eq!(lines.len(), rows.len());
            for (line, cells) in lines.iter().zip(&rows) {
                let read = split(line);
                prop_assert_eq!(read.len(), width as usize);
                for (index, cell) in read.iter().enumerate() {
                    let column = u32::try_from(index).unwrap() + 1;
                    let want = cells.get(&column).map_or("", String::as_str);
                    prop_assert_eq!(cell.as_str(), want);
                }
            }
        }
    }

    /// A comma-separated line's cells, double quotes undone: RFC 4180's rule, written out here
    /// on its own.
    fn split(line: &str) -> Vec<String> {
        let mut cells = vec![String::new()];
        let mut quoted = false;
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match (c, quoted) {
                ('"', true) if chars.peek() == Some(&'"') => {
                    chars.next();
                    cells.last_mut().unwrap().push('"');
                }
                ('"', _) => quoted = !quoted,
                (',', false) => cells.push(String::new()),
                (c, _) => cells.last_mut().unwrap().push(c),
            }
        }
        cells
    }
}
