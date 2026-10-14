//! Zip archives of a flight's files, as flyers share them: a Blue Raven's high-rate and low-rate
//! files with the app's summary, or a folder of everything a flight left behind.
//!
//! An archive isn't a log. [`unpack`] takes its files out, in memory, and each is then read as a
//! file on its own ([`import::read`](crate::import::read)); an archive inside an archive is one of
//! its files like any other, and is not opened. Nothing is written anywhere, so a file's name is
//! kept as the archive gives it, folders and all, and is only a label.
//!
//! An archive comes from someone else, so what comes out of it is bounded: at most
//! [`MAX_UNPACKED_BYTES`] of decompressed bytes, counted as they come out rather than as the
//! archive states them. (The zip reader's own record of the archive's directory, a few times the
//! directory's size, is on top.) Two files of one name, which the zip reader would silently keep
//! one of, are refused. A file is read with only the methods zip archives use for flight logs,
//! stored and deflated; an encrypted file, another method or a damaged file is refused with its
//! reason. The zip format is PKWARE's *APPNOTE.TXT*, version 6.3.10.
//!
//! ```
//! use std::io::{Cursor, Write};
//!
//! use hpr_flightdata::archive;
//!
//! // An invented archive of one PerfectFlite export.
//! let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
//! zip.start_file("flight/export.csv", zip::write::SimpleFileOptions::default())?;
//! zip.write_all(b"Time,Altitude,Velocity\n0.00,0,0\n0.05,1,8\n0.10,3,30\n")?;
//! let bytes = zip.finish()?.into_inner();
//!
//! let members = archive::unpack(&bytes)?;
//! assert_eq!(members[0].name, "flight/export.csv");
//! let log = hpr_flightdata::import::read(&members[0].bytes)?;
//! assert_eq!(log.time_s.len(), 3);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::io::{Cursor, Read};

use serde::{Deserialize, Serialize};

use crate::error::LogError;

/// The most bytes [`unpack`] takes out of one archive, 256 MiB (268 MB): about eight times the
/// largest of the project's sample archives, which unpacks to 32 MB, and the limit the `.hprz`
/// design container holds to.
pub const MAX_UNPACKED_BYTES: u64 = 256 * 1024 * 1024;

/// The format's name in errors.
const FORMAT: &str = "zip archive";

/// One file taken out of an archive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Member {
    /// The file's name as the archive gives it, folders before it separated by `/`.
    pub name: String,
    /// The file's bytes, decompressed.
    pub bytes: Vec<u8>,
}

/// Whether the bytes open as a zip archive does: a local file header, or the end of an empty
/// archive's central directory (APPNOTE.TXT 4.3.7, 4.3.16).
pub fn is_archive(bytes: &[u8]) -> bool {
    matches!(bytes, [b'P', b'K', 3, 4, ..] | [b'P', b'K', 5, 6, ..])
}

/// The files in a zip archive, in its central directory's order, folders' own entries left out;
/// [`unpack_within`] with [`MAX_UNPACKED_BYTES`].
///
/// # Errors
///
/// As [`unpack_within`].
pub fn unpack(bytes: &[u8]) -> Result<Vec<Member>, LogError> {
    unpack_within(bytes, MAX_UNPACKED_BYTES)
}

/// The files in a zip archive, decompressing at most `max_unpacked_bytes` out of it.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] when the bytes don't open as a zip archive; [`LogError::Archive`]
/// when the zip reader can't read its central directory, the directory holds more entries than
/// the reader keeps (two of one name), an entry is a symbolic link or a folder's entry that holds
/// bytes, a file is encrypted, compressed by a method other than stored or deflated, or damaged
/// (its CRC-32 fails), or the archive decompresses to more than `max_unpacked_bytes`.
pub fn unpack_within(bytes: &[u8], max_unpacked_bytes: u64) -> Result<Vec<Member>, LogError> {
    if !is_archive(bytes) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "its bytes don't open as a zip archive's, `PK` then 3, 4".to_owned(),
        });
    }
    let refused = |message: String| LogError::Archive { message };
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| refused(format!("its central directory can't be read: {error}")))?;
    // The zip reader keeps one entry of each name and drops the others without a word, so the
    // central directory's records are counted here: more records than entries means two share a
    // name, and one of them would be lost.
    let start = usize::try_from(archive.central_directory_start()).unwrap_or(usize::MAX);
    let records = central_records(bytes, start);
    if records != archive.len() {
        return Err(refused(format!(
            "its central directory holds {records} entries, and the zip reader keeps {}: two \
             have the same name, and one would be lost, or the directory is damaged",
            archive.len()
        )));
    }
    let mut members = Vec::new();
    let mut used = 0u64;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| refused(format!("entry {index} can't be opened: {error}")))?;
        let name = entry.name().to_owned();
        if entry.is_symlink() {
            return Err(refused(format!(
                "entry {index}, {:?}, is a symbolic link, not a file",
                shortened(&name)
            )));
        }
        let left = max_unpacked_bytes.saturating_sub(used);
        let mut content = Vec::new();
        // One byte past what is left, so that an entry filling the budget exactly is still known
        // to have overrun it. The archive's stated sizes aren't trusted: the bytes are counted as
        // they come out.
        entry
            .by_ref()
            .take(left.saturating_add(1))
            .read_to_end(&mut content)
            .map_err(|error| {
                refused(format!(
                    "entry {index}, {:?}, can't be decompressed: {error}",
                    shortened(&name)
                ))
            })?;
        if content.len() as u64 > left {
            return Err(refused(format!(
                "it decompresses to more than {max_unpacked_bytes} bytes, the most taken out of \
                 one archive"
            )));
        }
        used += content.len() as u64;
        if entry.is_dir() {
            // A folder's entry holds nothing: one that holds bytes, whatever size it states,
            // would be lost.
            if entry.size() > 0 || !content.is_empty() {
                return Err(refused(format!(
                    "entry {index}, {:?}, is a folder's entry that holds bytes",
                    shortened(&name)
                )));
            }
            continue;
        }
        // `read_to_end` grows by doubling: the slack goes back before the bytes are kept.
        content.shrink_to_fit();
        members.push(Member {
            name,
            bytes: content,
        });
    }
    Ok(members)
}

/// How many records a zip archive's central directory holds from `start`: each begins with the
/// signature `PK\x01\x02` and is 46 bytes, then its name, extra field and comment, whose lengths
/// are the little-endian `u16`s at 28, 30 and 32 (APPNOTE.TXT 6.3.10, section 4.3.12).
fn central_records(bytes: &[u8], start: usize) -> usize {
    const SIGNATURE: &[u8] = b"PK\x01\x02";
    const FIXED: usize = 46;
    let u16_at = |at: usize| usize::from(u16::from_le_bytes([bytes[at], bytes[at + 1]]));
    let mut count = 0;
    let mut at = start;
    while let Some(record) = bytes.get(at..at.saturating_add(FIXED))
        && record.starts_with(SIGNATURE)
    {
        count += 1;
        // In range: the fixed part was just read.
        at += FIXED + u16_at(at + 28) + u16_at(at + 30) + u16_at(at + 32);
    }
    count
}

/// A name cut to 60 characters for an error, so a hostile archive's 64 KiB name isn't copied
/// into every message.
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
    use std::io::Write;

    use zip::write::SimpleFileOptions;

    use super::*;

    /// An archive of the given files, each deflated.
    fn archive(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for (name, bytes) in files {
            zip.start_file(*name, SimpleFileOptions::default())
                .expect("an in-memory archive takes a file");
            zip.write_all(bytes).expect("an in-memory write succeeds");
        }
        zip.finish()
            .expect("an in-memory archive finishes")
            .into_inner()
    }

    fn message(result: Result<Vec<Member>, LogError>) -> String {
        match result {
            Err(LogError::Archive { message }) => message,
            other => panic!("expected an archive refusal, got {other:?}"),
        }
    }

    #[test]
    fn files_come_out_in_order_with_their_names_and_folders_left_out() {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        zip.add_directory("Data/", SimpleFileOptions::default())
            .expect("a folder entry is written");
        for (name, bytes) in [("Data/b.csv", &b"2"[..]), ("Data/a.csv", b"1")] {
            zip.start_file(name, SimpleFileOptions::default())
                .expect("a file entry is written");
            zip.write_all(bytes).expect("an in-memory write succeeds");
        }
        let bytes = zip.finish().expect("finished").into_inner();
        let members = unpack(&bytes).expect("a plain archive unpacks");
        let names: Vec<&str> = members.iter().map(|member| member.name.as_str()).collect();
        assert_eq!(names, ["Data/b.csv", "Data/a.csv"]);
        assert_eq!(members[1].bytes, b"1");
    }

    #[test]
    fn an_archive_inside_is_a_file_like_any_other() {
        let inner = archive(&[("log.csv", b"x")]);
        let members = unpack(&archive(&[("inner.zip", &inner)])).expect("unpacks");
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].bytes, inner);
    }

    #[test]
    fn the_budget_counts_the_bytes_that_come_out() {
        let bytes = archive(&[("a", &[0u8; 600][..]), ("b", &[0u8; 400][..])]);
        assert!(unpack_within(&bytes, 1000).is_ok());
        // One byte short: the second file overruns it, though zeros compress to a few bytes.
        let refusal = message(unpack_within(&bytes, 999));
        assert!(refusal.contains("more than 999 bytes"), "{refusal}");
    }

    #[test]
    fn a_stated_size_smaller_than_the_data_does_not_fool_the_budget() {
        // A file of 4,000 zeros whose central directory states 10 bytes, uncompressed: a bomb's
        // trick. Its size fields are at offset 24 of the central record and 22 of the local one.
        let mut bytes = archive(&[("bomb", &[0u8; 4000][..])]);
        let central = bytes
            .windows(4)
            .position(|window| window == b"PK\x01\x02")
            .expect("the archive has a central directory");
        bytes[central + 24..central + 28].copy_from_slice(&10u32.to_le_bytes());
        bytes[22..26].copy_from_slice(&10u32.to_le_bytes());
        let refusal = message(unpack_within(&bytes, 1000));
        assert!(refusal.contains("more than 1000 bytes"), "{refusal}");
    }

    #[test]
    fn two_files_of_one_name_are_refused() {
        // Written under two names of one length, then the second renamed to the first, in its
        // local header and its central record alike.
        let mut bytes = archive(&[("a.csv", b"1"), ("b.csv", b"2")]);
        let positions: Vec<usize> = bytes
            .windows(5)
            .enumerate()
            .filter(|(_, window)| *window == b"b.csv")
            .map(|(at, _)| at)
            .collect();
        assert_eq!(positions.len(), 2);
        for at in positions {
            bytes[at] = b'a';
        }
        let refusal = message(unpack(&bytes));
        assert!(refusal.contains("same name"), "{refusal}");
    }

    /// An archive of one file stored, not compressed.
    fn stored(name: &str, bytes: &[u8]) -> Vec<u8> {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip.start_file(name, options)
            .expect("a stored file is written");
        zip.write_all(bytes).expect("an in-memory write succeeds");
        zip.finish().expect("finished").into_inner()
    }

    /// The offset of the central directory's first record.
    fn central(bytes: &[u8]) -> usize {
        bytes
            .windows(4)
            .position(|window| window == b"PK\x01\x02")
            .expect("the archive has a central directory")
    }

    #[test]
    fn a_damaged_file_is_refused_by_its_checksum() {
        // A stored file's bytes follow the 30-byte local header and the 5-byte name as they are,
        // so a flipped byte there reaches the CRC-32 and nothing else.
        let mut bytes = stored("a.csv", b"Time,Altitude\n0,0\n");
        bytes[30 + 5] ^= 0x01;
        let refusal = message(unpack(&bytes));
        assert!(refusal.contains("can't be decompressed"), "{refusal}");
        assert!(refusal.to_lowercase().contains("checksum"), "{refusal}");
    }

    #[test]
    fn a_symbolic_link_is_refused() {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        zip.add_symlink("link.csv", "/etc/passwd", SimpleFileOptions::default())
            .expect("a link is written");
        let bytes = zip.finish().expect("finished").into_inner();
        let refusal = message(unpack(&bytes));
        assert!(refusal.contains("symbolic link"), "{refusal}");
    }

    #[test]
    fn an_encrypted_file_is_refused() {
        // Bit 0 of the general-purpose flags, at offset 6 of the local header and 8 of the
        // central record, marks a file encrypted (APPNOTE.TXT 4.4.4).
        let mut bytes = stored("a.csv", b"0,0\n");
        let at = central(&bytes);
        bytes[6] |= 1;
        bytes[at + 8] |= 1;
        let refusal = message(unpack(&bytes));
        assert!(refusal.contains("assword"), "{refusal}");
    }

    #[test]
    fn a_method_other_than_stored_or_deflated_is_refused() {
        // The method, at offset 8 of the local header and 10 of the central record, set to 12,
        // bzip2 (APPNOTE.TXT 4.4.5), which the zip reader is built without.
        let mut bytes = stored("a.csv", b"0,0\n");
        let at = central(&bytes);
        bytes[8..10].copy_from_slice(&12u16.to_le_bytes());
        bytes[at + 10..at + 12].copy_from_slice(&12u16.to_le_bytes());
        let refusal = message(unpack(&bytes));
        assert!(refusal.contains("not supported"), "{refusal}");
    }

    #[test]
    fn a_folder_entry_that_holds_bytes_is_refused_whatever_size_it_states() {
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        zip.start_file("dd/", SimpleFileOptions::default())
            .expect("an entry named as a folder is written");
        zip.write_all(&[b'x'; 500])
            .expect("an in-memory write succeeds");
        let mut bytes = zip.finish().expect("finished").into_inner();
        // As written, it states its 500 bytes.
        assert!(message(unpack(&bytes)).contains("folder's entry that holds bytes"));
        // With its stated size, at offset 24 of the central record, set to 0, it still holds them.
        let at = central(&bytes);
        bytes[at + 24..at + 28].copy_from_slice(&0u32.to_le_bytes());
        let refusal = message(unpack(&bytes));
        assert!(
            refusal.contains("folder's entry that holds bytes"),
            "{refusal}"
        );
    }

    #[test]
    fn bytes_that_are_not_an_archive_are_not_this_format() {
        assert!(matches!(
            unpack(b"Time,Altitude\n"),
            Err(LogError::NotThisFormat { .. })
        ));
        assert!(matches!(
            unpack(b"PK\x03\x04 and nothing more"),
            Err(LogError::Archive { .. })
        ));
        assert!(!is_archive(b"PK"));
        assert!(is_archive(b"PK\x05\x06"));
    }

    #[test]
    fn an_empty_archive_holds_no_files() {
        let bytes = archive(&[]);
        assert!(is_archive(&bytes));
        assert_eq!(unpack(&bytes), Ok(Vec::new()));
    }

    proptest::proptest! {
        /// Hostile bytes after a zip's opening are refused or unpacked within the budget, never a
        /// panic.
        #[test]
        fn any_bytes_are_refused_or_held_to_the_budget(
            tail in proptest::collection::vec(proptest::num::u8::ANY, 0..600),
        ) {
            let mut bytes = b"PK\x03\x04".to_vec();
            bytes.extend(tail);
            if let Ok(members) = unpack_within(&bytes, 4096) {
                let total: usize = members.iter().map(|member| member.bytes.len()).sum();
                proptest::prop_assert!(total <= 4096);
            }
        }

        /// A real archive with bytes overwritten anywhere is refused or held to the budget.
        #[test]
        fn a_damaged_archive_is_refused_or_held_to_the_budget(
            edits in proptest::collection::vec((0usize..400, proptest::num::u8::ANY), 1..8),
        ) {
            let mut bytes = archive(&[("a.csv", &[b'7'; 300][..]), ("b/c.txt", b"0.0\t1\t2F\t3")]);
            for (at, value) in edits {
                let at = at % bytes.len();
                bytes[at] = value;
            }
            if let Ok(members) = unpack_within(&bytes, 4096) {
                let total: usize = members.iter().map(|member| member.bytes.len()).sum();
                proptest::prop_assert!(total <= 4096);
            }
        }
    }

    #[test]
    fn a_long_name_is_shortened_in_errors() {
        let name = "x".repeat(1000);
        assert_eq!(shortened(&name).chars().count(), 61);
        assert_eq!(shortened("short"), "short");
    }
}
