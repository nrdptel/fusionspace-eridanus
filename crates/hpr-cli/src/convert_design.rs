//! `hpr convert` of a design: a `.ork`, `.hpr` or `.hprz` file written as any of the three.
//!
//! The conversion is the library's own ([`hpr::hpr_format`]). A `.ork` is read into a document of
//! the HPR design format ([`DesignFile::from_ork`]), which names the file's SHA-256 as its source;
//! a `.hpr` or `.hprz` of an older version is migrated to the current one. The document is written
//! as its canonical text, in a container with its attachments, or as the `.ork` HPR Sim writes from
//! it. A conversion keeps where the design came from as read, its provenance's `source`, which
//! rewriting it doesn't change (ADR-112); the program that wrote the file is this one, at this
//! version, with its designation ([`DesignFile::stamped`], ADR-174).

use std::io::{self, Write};
use std::path::Path;

use hpr::hpr_format::container::{self, Entry, Hprz};
use hpr::hpr_format::{self, DesignFile};

use crate::console::{Diagnostics, Level};
use crate::convert::{ConvertArgs, file_name, output_folder};
use crate::output::{
    Convert, ConvertDesign, DesignFilePath, DesignFormat, InputWarning, WarningKind,
};
use crate::sim::{ork_warning, same_file};
use crate::{Failure, Out};

/// The design formats `hpr convert` reads and writes, by a path's extension.
fn format_of(path: &str) -> Option<DesignFormat> {
    let extension = Path::new(path).extension()?.to_str()?.to_ascii_lowercase();
    match extension.as_str() {
        "ork" => Some(DesignFormat::Ork),
        "hpr" => Some(DesignFormat::Hpr),
        "hprz" => Some(DesignFormat::Hprz),
        _ => None,
    }
}

/// Whether `path` names a design file `hpr convert` reads or writes.
pub(crate) fn is_design(path: &str) -> bool {
    format_of(path).is_some()
}

/// A design read: the document, the version it was written in, a container's attachments, and
/// what the reader flagged.
struct ReadDesign {
    document: DesignFile,
    written_as: hpr_format::Version,
    attachments: Vec<Entry>,
    warnings: Vec<InputWarning>,
}

/// Runs `hpr convert` for a design.
pub(crate) fn run(args: &ConvertArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    let (input, output) = (&args.input, &args.output);
    let (Some(from), Some(target)) = (format_of(input), format_of(output)) else {
        let help = if is_design(input) {
            "end the output's name in .ork, .hpr or .hprz to write the design"
        } else {
            "give a .ork, .hpr or .hprz design to write a design, or end the output's name in \
             .eng or .rse to write motors"
        };
        return Err(Failure::helped(
            format!(
                "{input} to {output}: hpr convert writes a design (.ork, .hpr or .hprz) from a \
                 design, and motors (.eng or .rse) from motors"
            ),
            help,
        ));
    };
    if args.delays.is_some() {
        return Err(Failure::helped(
            "--delays gives a .rse motor's delays for a .eng file, and hpr convert is writing a \
             design",
            "leave out --delays",
        ));
    }
    if !args.attach.is_empty() && target != DesignFormat::Hprz {
        return Err(Failure::helped(
            format!("{output}: --attach adds files to a .hprz, and this is not one"),
            format!(
                "write {} to carry the files, or leave out --attach",
                Path::new(output).with_extension("hprz").display()
            ),
        ));
    }
    if same_file(input) == same_file(output) {
        return Err(Failure::helped(
            format!("{output}: that is the file hpr convert reads, and it would write over it"),
            "name another file to write",
        ));
    }
    output_folder(output)?;
    let mut read = read(input, from)?;
    read.document = read.document.stamped();
    let mut warnings = read.warnings;
    let mut attachments = read.attachments;
    for path in &args.attach {
        let name = file_name(path);
        container::check_name(&name).map_err(|why| {
            Failure::helped(
                format!("{path}: {why}"),
                "copy the file to a plain name, such as notes.txt, and attach the copy",
            )
        })?;
        let bytes = crate::read_file(path)?;
        attachments.push(Entry::new(name, bytes));
    }
    let unwritable = |error: &dyn std::fmt::Display, help: &str| {
        Failure::helped(
            format!("{output}: the design can't be written: {error}"),
            help,
        )
    };
    let (bytes, written) = match target {
        DesignFormat::Hprz => {
            let names = attachments.iter().map(|entry| entry.name.clone()).collect();
            // With no attachments, the container holds only the design, which was read.
            let help = if attachments.is_empty() {
                crate::BUG_HELP
            } else {
                "change or leave out the attachment the message names"
            };
            let hprz = Hprz::new(read.document.clone(), attachments);
            (
                container::write(&hprz).map_err(|e| unwritable(&e, help))?,
                names,
            )
        }
        _ => {
            // A `.hpr` or `.ork` has no place for a container's own files.
            for entry in &attachments {
                warnings.push(InputWarning {
                    at: entry.name.clone(),
                    kind: WarningKind::Dropped,
                    message: format!(
                        "the {} file has no place for a .hprz's attachments, so this one was left \
                         out",
                        extension(target)
                    ),
                });
            }
            if target == DesignFormat::Hpr {
                // A design read is one its own format can write.
                let text = hpr_format::to_json(&read.document)
                    .map_err(|e| unwritable(&e, crate::BUG_HELP))?;
                (text.into_bytes(), Vec::new())
            } else {
                let ork = read
                    .document
                    .to_ork()
                    .map_err(|e| unwritable(&e, "write it as a .hpr or .hprz file instead"))?;
                warnings.extend(ork.warnings.iter().map(ork_warning));
                (ork.value, Vec::new())
            }
        }
    };
    std::fs::write(output, bytes).map_err(|error| crate::unwritable(output, &error))?;
    let document = ConvertDesign {
        input: DesignFilePath {
            path: input.clone(),
            format: from,
        },
        output: DesignFilePath {
            path: output.clone(),
            format: target,
        },
        rocket: read.document.rocket.name.clone(),
        configurations: read.document.motors.configurations.len(),
        migrated_from: (read.written_as != hpr_format::VERSION)
            .then(|| read.written_as.to_string()),
        attachments: written,
        warnings,
    };
    to.emit(&Convert::Design(document.clone()), |out, diagnostics| {
        text_output(&document, out, diagnostics)
    })
}

/// Reads the design at `path`, of `format`.
fn read(path: &str, format: DesignFormat) -> Result<ReadDesign, Failure> {
    let bytes = crate::read_file(path)?;
    let refused = |error: &dyn std::fmt::Display, help: String| {
        Failure::helped(format!("{path}: {error}"), help)
    };
    match format {
        DesignFormat::Ork => {
            let read = DesignFile::from_ork(&bytes).map_err(|e| {
                refused(
                    &e,
                    "give a .ork file OpenRocket saved; if this is one, save it again from \
                     OpenRocket and convert the copy"
                        .to_owned(),
                )
            })?;
            Ok(ReadDesign {
                document: read.value,
                written_as: hpr_format::VERSION,
                attachments: Vec::new(),
                warnings: read.warnings.iter().map(ork_warning).collect(),
            })
        }
        DesignFormat::Hprz => {
            let opened = container::read(&bytes).map_err(|e| refused(&e, read_help(&e)))?;
            Ok(ReadDesign {
                document: opened.value.design,
                written_as: opened.written_as,
                attachments: opened.value.attachments,
                warnings: Vec::new(),
            })
        }
        _ => {
            let text = String::from_utf8(bytes).map_err(|_| {
                Failure::helped(
                    format!("{path}: an .hpr file is UTF-8 text"),
                    "give a .hpr design HPR Sim wrote; this file isn't one",
                )
            })?;
            let opened = hpr_format::read_json(&text).map_err(|e| refused(&e, read_help(&e)))?;
            Ok(ReadDesign {
                document: opened.value,
                written_as: opened.written_as,
                attachments: Vec::new(),
                warnings: Vec::new(),
            })
        }
    }
}

/// What to do about a `.hpr` or `.hprz` design that doesn't read, by why.
fn read_help(error: &hpr_format::FormatError) -> String {
    use hpr_format::FormatError;
    match error {
        FormatError::Unsupported {
            found, supported, ..
        } if found > supported => "a newer HPR Sim wrote it; update HPR Sim to read it".to_owned(),
        FormatError::Unsupported { .. } => "an HPR Sim older than this one migrates from wrote \
             it; convert it with that HPR Sim to a newer version first"
            .to_owned(),
        FormatError::Json(_) | FormatError::NotADesign { .. } => {
            "give a .hpr or .hprz design HPR Sim wrote; this file isn't one".to_owned()
        }
        _ => "fix what the message names, or convert the design again from the .ork it came \
              from"
            .to_owned(),
    }
}

/// A design format's extension, with its dot.
fn extension(format: DesignFormat) -> &'static str {
    match format {
        DesignFormat::Ork => ".ork",
        DesignFormat::Hpr => ".hpr",
        DesignFormat::Hprz => ".hprz",
        DesignFormat::HprJson => ".json",
    }
}

/// `hpr convert` of a design as text: what was read and what was written, and the warnings on
/// standard error.
fn text_output(
    document: &ConvertDesign,
    out: &mut dyn Write,
    diagnostics: &mut Diagnostics<'_>,
) -> io::Result<()> {
    let migrated = document
        .migrated_from
        .as_ref()
        .map(|version| format!(", migrated from version {version}"))
        .unwrap_or_default();
    writeln!(out, "read   {}{migrated}", file_name(&document.input.path))?;
    let configurations = match document.configurations {
        1 => "1 motor configuration".to_owned(),
        n => format!("{n} motor configurations"),
    };
    let version = match document.output.format {
        DesignFormat::Hpr | DesignFormat::Hprz => {
            format!(", HPR design format {}", hpr_format::VERSION)
        }
        _ => String::new(),
    };
    writeln!(
        out,
        "wrote  {}: {:?}, {configurations}{version}",
        file_name(&document.output.path),
        document.rocket,
    )?;
    if !document.attachments.is_empty() {
        writeln!(out, "with   {}", document.attachments.join(", "))?;
    }
    for warning in &document.warnings {
        diagnostics.line(
            Level::Warning,
            &format!("{}: {}", warning.at, warning.message),
        )?;
    }
    Ok(())
}
