//! One entry point for every format HPR Sim reads: give [`read`] a file's bytes, and it
//! recognizes the layout from the content, never the file's name, and reads it into the record
//! every reader fills ([`FlightLog`]).
//!
//! The layouts it recognizes, the raw downloads by their bytes, the GPS logs by their own marks,
//! the rest by the first line that isn't blank:
//!
//! | Format | Recognized by |
//! |---|---|
//! | [Raven `.FIPa`](crate::raven_fipa) | its bytes: they open `#Component=Raven`, and a `#den` line within the first 4,096 is followed by a frame's `:`; told before the file is decoded as text |
//! | [RRC3 `.rff`](crate::rrc3_rff) | its bytes: an MS-NRBF serialization header, version 1.0, and mDACS's `mDACS.AnalysisSupport+FlightRecord` class name within the first 4,096; told before the file is decoded as text |
//! | [KML](crate::kml) | its root element is `kml`, after any XML declaration and comments |
//! | [NMEA 0183](crate::nmea) | at least 3 of its first 50 lines that aren't blank open with a sentence whose checksum checks out, such as `$GPGGA,…*hh` |
//! | [Featherweight GPS CSV](crate::fwgps) | the first line opens `UTCTIME,UNIXTIME,` or `TRACKER,DATE,TIME,` and names the latitude and longitude |
//! | [PerfectFlite `.pf2`](crate::perfectflite) | names PerfectFlite, such as `PerfectFlite Pnut`, in a file with no `**!!Data:` line (a `.pfa`'s) |
//! | [PerfectFlite `.pfa`](crate::perfectflite::read_pfa) | names PerfectFlite, such as `PerfectFlite Alt15K/WD`, and has a `**!!Data:` line |
//! | [PerfectFlite CSV and text](crate::perfectflite::read_export) | `Time`, `Altitude` and more of an export's names, by commas or tabs |
//! | [AltOS CSV](crate::altos) | starts `#version,`, or opens with `#` comments before a `#version,` or `version,serial,` header |
//! | [Raven FIP CSV](crate::raven) | opens with a time column and holds the Raven's `Axial Accel (Gs)` channel |
//! | [Blue Raven CSV](crate::blueraven) | opens `Year,Month,Day,Time,Flight_Time_(s),Sync,` |
//! | [RRC3 CSV](crate::rrc3) | `Time`, `Altitude`, `Pressure`, `Velocity`, then any of `Temperature`, `Events`, `Voltages` |
//! | [AltimeterCloud CSV](crate::altimetercloud) | a header within the first 65 lines whose first name is `Time(ms)`, in any case, and which also names `acceleration_total(mG)` and `tilt` |
//! | [Entacore AIM CSV](crate::entacore) | the first line starts with a `time` column, names at least two, and names one of the AIM's own channels, such as `Pressure AGL` |
//! | [ARTS2 CSV](crate::arts2) | a first line of `#` and the names `MET Acc Vel iAlt pAlt`, by spaces, tabs or commas, in any case; tried before AltOS's `#` comments |
//! | [SillyGoose text](crate::sillygoose) | the first line, or the one after a `CONFIG` line, starts with `timestampMs` and names `pressurePa` and `altitudeM` |
//! | [Silicdyne Fluctus CSV](crate::fluctus) | a `sep=` line, or none, then a header starting `time` that names `deltaTime` and `baro-altitude` |
//! | [Eggtimer text](crate::eggtimer::read_text) | an `Eggtimer … Flight Settings` title before an Eggtimer CSV header, within the first 200 lines that aren't blank |
//! | [Eggtimer CSV](crate::eggtimer) | `T,Alt,` and more of Eggtimer's column names |
//!
//! Text is read as UTF-8, or as UTF-16 after its byte-order mark ([`decode`]), with any byte-order
//! mark dropped; a byte that isn't UTF-8 becomes U+FFFD, as a `.pf2`'s comments may be in another
//! encoding and no reader uses them.
//!
//! ```
//! use hpr_flightdata::{import, log::LogFormat};
//!
//! let text = "Time,Altitude,Velocity\n0.00,0,0\n0.05,1,8\n0.10,3,30\n";
//! assert_eq!(import::detect(text.as_bytes()), Some(LogFormat::PerfectFliteCsv));
//! let log = import::read(text.as_bytes())?;
//! assert_eq!(log.channels.len(), 2); // the altitude and the velocity; the time is their clock
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```

use std::borrow::Cow;

use crate::error::LogError;
use crate::log::{FlightLog, LogFormat};
use crate::{
    aerospacenu_fcb, aerospacenu_gs, altimetercloud, altos, arts2, blueraven, eggtimer, entacore,
    fluctus, fwgps, kml, nmea, perfectflite, phone_launch, raven, raven_fipa, rrc3, rrc3_rff,
    sillygoose,
};

/// The formats [`read`] recognizes, in the order it tries them.
pub const FORMATS: &[LogFormat] = &[
    LogFormat::RavenFipa,
    LogFormat::Rrc3Rff,
    LogFormat::EggtimerRtf,
    LogFormat::AerospaceNuGroundStationText,
    LogFormat::Kml,
    LogFormat::Nmea0183,
    LogFormat::FeatherweightGpsCsv,
    LogFormat::Arts2Csv,
    LogFormat::PerfectFlitePf2,
    LogFormat::PerfectFlitePfa,
    LogFormat::AltosCsv,
    LogFormat::RavenFipCsv,
    LogFormat::BlueRavenCsv,
    LogFormat::Rrc3Csv,
    LogFormat::AltimeterCloudCsv,
    LogFormat::EntacoreAimCsv,
    LogFormat::SillyGooseText,
    LogFormat::FluctusCsv,
    LogFormat::AerospaceNuFcbCsv,
    LogFormat::PhoneLaunchCsv,
    LogFormat::EggtimerText,
    LogFormat::EggtimerCsv,
    LogFormat::PerfectFliteCsv,
    LogFormat::PerfectFliteText,
];

/// A file's bytes as text: UTF-8, or UTF-16 if the file opens with UTF-16's byte-order mark, as
/// some mDACS exports of an RRC3's flight are; any byte-order mark dropped, and a byte, or a pair
/// of them, that isn't a character replaced by U+FFFD. One such export, among the project's sample
/// files, opens after its UTF-16 mark with a UTF-8 mark's three bytes, each written as a UTF-16
/// character (`ï»¿`): they are dropped too, as the mark they are.
pub fn decode(bytes: &[u8]) -> Cow<'_, str> {
    let utf16 = |bytes: &[u8], unit: fn([u8; 2]) -> u16| {
        let (pairs, odd) = bytes.as_chunks::<2>();
        // An odd byte at the end is half a character.
        let odd = (!odd.is_empty()).then_some(char::REPLACEMENT_CHARACTER);
        let text: String = char::decode_utf16(pairs.iter().map(|pair| unit(*pair)))
            .map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER))
            .chain(odd)
            .collect();
        Cow::Owned(match text.strip_prefix("\u{ef}\u{bb}\u{bf}") {
            Some(rest) => rest.to_owned(),
            None => text,
        })
    };
    if let Some(rest) = bytes.strip_prefix(b"\xff\xfe") {
        utf16(rest, u16::from_le_bytes)
    } else if let Some(rest) = bytes.strip_prefix(b"\xfe\xff") {
        utf16(rest, u16::from_be_bytes)
    } else {
        let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes);
        String::from_utf8_lossy(bytes)
    }
}

/// The format of a log, recognized from its first line that isn't blank; `None` for a layout HPR
/// Sim doesn't read.
pub fn detect(bytes: &[u8]) -> Option<LogFormat> {
    detect_binary(bytes).or_else(|| detect_text(&decode(bytes)))
}

/// The binary formats, told by their bytes: their text would be mojibake.
fn detect_binary(bytes: &[u8]) -> Option<LogFormat> {
    if raven_fipa::recognizes(bytes) {
        Some(LogFormat::RavenFipa)
    } else if rrc3_rff::recognizes(bytes) {
        Some(LogFormat::Rrc3Rff)
    } else {
        None
    }
}

fn detect_text(text: &str) -> Option<LogFormat> {
    // Lines end in CR, LF or both, as the readers take them.
    let mut lines = text
        .split(['\r', '\n'])
        .map(str::trim)
        .filter(|line| !line.is_empty());
    let first = lines.next()?;
    // An RTF document is told by its opening group; only an Eggtimer's download is read in one.
    if eggtimer::recognizes_rtf(text) {
        return Some(LogFormat::EggtimerRtf);
    }
    if aerospacenu_gs::recognizes(text) {
        return Some(LogFormat::AerospaceNuGroundStationText);
    }
    // A GPS log is told by its own marks, which need not be on its first line: an NMEA file may
    // open with the receiver's power-up noise, a KML file with an XML declaration.
    if kml::recognizes(text) {
        return Some(LogFormat::Kml);
    }
    if nmea::recognizes(text) {
        return Some(LogFormat::Nmea0183);
    }
    if fwgps::recognizes(text) {
        return Some(LogFormat::FeatherweightGpsCsv);
    }
    let altos_header =
        |line: &str| line.starts_with("#version,") || line.starts_with("version,serial,");
    // ARTS2's header opens with `#`, which would otherwise be taken for AltOS's comments.
    if arts2::recognizes(text) {
        return Some(LogFormat::Arts2Csv);
    }
    if first.to_ascii_lowercase().contains("perfectflite") {
        // A `.pfa` names PerfectFlite too, but writes its samples after a `**!!Data:` line, one
        // value a line.
        let pfa = text
            .split(['\r', '\n'])
            .any(|line| line.trim_start().starts_with("**!!"));
        Some(if pfa {
            LogFormat::PerfectFlitePfa
        } else {
            LogFormat::PerfectFlitePf2
        })
    } else if first.starts_with('#') && !altos_header(first) {
        // Only AltOS's CSV opens with `#` comments, such as `# Comments` or `# product TeleMetrum`,
        // before its header.
        std::iter::once(first)
            .chain(lines)
            .find(|line| !line.starts_with('#') || altos_header(line))
            .filter(|line| altos_header(line))
            .map(|_| LogFormat::AltosCsv)
    } else if altos_header(first) {
        Some(LogFormat::AltosCsv)
    } else if first.starts_with("Time") && first.contains("Axial Accel (Gs)") {
        // The Raven's export opens with a time column; a Blue Raven's names the same channel
        // further along a header that opens with the date.
        Some(LogFormat::RavenFipCsv)
    } else if blueraven::recognizes(text) {
        Some(LogFormat::BlueRavenCsv)
    } else if rrc3::recognizes(text) {
        Some(LogFormat::Rrc3Csv)
    } else if altimetercloud::recognizes(text) {
        Some(LogFormat::AltimeterCloudCsv)
    } else if entacore::recognizes(text) {
        Some(LogFormat::EntacoreAimCsv)
    } else if sillygoose::recognizes(text) {
        Some(LogFormat::SillyGooseText)
    } else if fluctus::recognizes(text) {
        Some(LogFormat::FluctusCsv)
    } else if aerospacenu_fcb::recognizes(text) {
        Some(LogFormat::AerospaceNuFcbCsv)
    } else if phone_launch::recognizes(text) {
        Some(LogFormat::PhoneLaunchCsv)
    } else if eggtimer::recognizes_text(text) {
        Some(LogFormat::EggtimerText)
    } else if eggtimer::recognizes(text) {
        Some(LogFormat::EggtimerCsv)
    } else {
        match perfectflite::export_separator(first)? {
            ',' => Some(LogFormat::PerfectFliteCsv),
            _ => Some(LogFormat::PerfectFliteText),
        }
    }
}

/// Reads a flight log in any format HPR Sim reads.
///
/// # Errors
///
/// [`LogError::Unrecognized`] for a layout HPR Sim doesn't read, naming those it does; otherwise
/// whatever the format's reader refuses.
pub fn read(bytes: &[u8]) -> Result<FlightLog, LogError> {
    // A binary file isn't decoded as text: its bytes are the reader's.
    let binary = detect_binary(bytes);
    let text = if binary.is_some() {
        Cow::Borrowed("")
    } else {
        decode(bytes)
    };
    match binary.or_else(|| detect_text(&text)) {
        Some(LogFormat::PerfectFlitePf2) => perfectflite::read(&text),
        Some(LogFormat::PerfectFlitePfa) => perfectflite::read_pfa(&text),
        Some(LogFormat::PerfectFliteCsv | LogFormat::PerfectFliteText) => {
            perfectflite::read_export(&text)
        }
        Some(LogFormat::AltosCsv) => altos::read(&text),
        Some(LogFormat::RavenFipCsv) => raven::read(&text),
        Some(LogFormat::BlueRavenCsv) => blueraven::read(&text),
        Some(LogFormat::Rrc3Csv) => rrc3::read(&text),
        Some(LogFormat::EggtimerCsv) => eggtimer::read(&text),
        Some(LogFormat::EggtimerText) => eggtimer::read_text(&text),
        Some(LogFormat::AltimeterCloudCsv) => altimetercloud::read(&text),
        Some(LogFormat::EntacoreAimCsv) => entacore::read(&text),
        Some(LogFormat::SillyGooseText) => sillygoose::read(&text),
        Some(LogFormat::FluctusCsv) => fluctus::read(&text),
        Some(LogFormat::Arts2Csv) => arts2::read(&text),
        Some(LogFormat::FeatherweightGpsCsv) => fwgps::read(&text),
        Some(LogFormat::Kml) => kml::read(&text),
        Some(LogFormat::Nmea0183) => nmea::read(&text),
        Some(LogFormat::AerospaceNuFcbCsv) => aerospacenu_fcb::read(&text),
        Some(LogFormat::AerospaceNuGroundStationText) => aerospacenu_gs::read(&text),
        Some(LogFormat::PhoneLaunchCsv) => phone_launch::read(&text),
        Some(LogFormat::EggtimerRtf) => eggtimer::read_rtf(&text),
        Some(LogFormat::RavenFipa) => raven_fipa::read(bytes),
        Some(LogFormat::Rrc3Rff) => rrc3_rff::read(bytes),
        None => {
            let known: Vec<&str> = FORMATS.iter().map(|format| format.name()).collect();
            Err(LogError::Unrecognized {
                message: format!(
                    "the file is in none of the layouts HPR Sim reads so far: {}",
                    known.join(", ")
                ),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_format_is_recognized_by_its_marks() {
        let cases = [
            ("PerfectFlite Pnut\n", Some(LogFormat::PerfectFlitePf2)),
            (
                "\u{feff}\n\n#version,serial,flight,time\n",
                Some(LogFormat::AltosCsv),
            ),
            (
                "# Altus Metrum flight log\n# serial 1\nversion,serial,flight,time\n",
                Some(LogFormat::AltosCsv),
            ),
            ("#Component=Raven\n", None),
            ("# MET Acc Vel iAlt pAlt\n", Some(LogFormat::Arts2Csv)),
            ("MET,Acc(HI),Acc/Baro(LO),Baro(HI),State\n", None),
            (
                "sep=;\ntime (ms);deltaTime (ms);status;baro-altitude (m)\n",
                Some(LogFormat::FluctusCsv),
            ),
            ("T;Alt;Veloc;FAlt\n", None),
            (
                "timestampMs\tpressurePa\ttempK\taltitudeM\n",
                Some(LogFormat::SillyGooseText),
            ),
            (
                "#foo\n#version,serial,flight,time\n",
                Some(LogFormat::AltosCsv),
            ),
            ("# a note\nTime,Altitude,Velocity\n", None),
            (
                "PerfectFlite Alt15K/WD\nApogee: 900\n**!!Data:\n1200\n",
                Some(LogFormat::PerfectFlitePfa),
            ),
            (
                "Year,Month,Day,Time,Flight_Time_(s),Axial Accel (Gs)\n",
                None,
            ),
            (
                "Time,Altitude,Velocity\r0.00,0,0\r0.05,1,8\r",
                Some(LogFormat::PerfectFliteCsv),
            ),
            (
                "Time@Axial Accel (Gs),Axial Accel (Gs),bILBA\n",
                Some(LogFormat::RavenFipCsv),
            ),
            (
                "Time,LiftOff,Axial Accel (Gs),Baro (Atm)\n",
                Some(LogFormat::RavenFipCsv),
            ),
            (
                "Time, Altitude, Velocity\n",
                Some(LogFormat::PerfectFliteCsv),
            ),
            (
                "Time\tAltitude\tTemp.\tVoltage\n",
                Some(LogFormat::PerfectFliteText),
            ),
            // A team's own export in feet: the generic reader's, M7.1c.
            ("time_s,altitude_ft,temperature,voltage\n", None),
            ("Time,Pressure\n", None),
            ("time,altitude\n", None),
            ("Time,Altitude\n", None),
            (
                "Time,Altitude,Pressure,Velocity,Temperature,Events,Voltages\n",
                Some(LogFormat::Rrc3Csv),
            ),
            (
                "T,Alt,Veloc,FAlt,FVeloc,LDA,LowV,Apogee,N-O,Drogue,Main\r\r\n",
                Some(LogFormat::EggtimerCsv),
            ),
            (
                "Year,Month,Day,Time,Flight_Time_(s),Sync,Gyro_X\n",
                Some(LogFormat::BlueRavenCsv),
            ),
            (
                "<?xml version=\"1.0\"?>\n<!-- a track -->\n<kml xmlns=\"http://www.opengis.net/kml/2.2\"></kml>\n",
                Some(LogFormat::Kml),
            ),
            ("<?xml version=\"1.0\"?>\n<gpx></gpx>\n", None),
            (
                "\u{fffd}\u{fffd}x\r\n\
                 $GPGGA,120000.00,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*67\r\n\
                 $GPRMC,120001.00,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*4B\r\n\
                 $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n",
                Some(LogFormat::Nmea0183),
            ),
            // A sentence quoted inside another log's line isn't NMEA.
            (
                "t=1 rx $GPGGA,120000.00,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*67\n",
                None,
            ),
            (
                "UTCTIME,UNIXTIME,ALT,LAT,LON,#SATS,FIX,HORZV,VERTV,HEAD,FLAGS\n",
                Some(LogFormat::FeatherweightGpsCsv),
            ),
            (
                "TRACKER,DATE,TIME,LAT,LON,ALT,FIX,HORZV,VERTV,HEAD,FLAGS,#TOT\r\n",
                Some(LogFormat::FeatherweightGpsCsv),
            ),
            ("TRACKER,DATE,TIME,RSSI\n", None),
            ("", None),
        ];
        for (text, format) in cases {
            assert_eq!(detect(text.as_bytes()), format, "{text:?}");
        }
    }

    #[test]
    fn utf16_is_read_by_its_byte_order_mark() {
        let text = "Time\tAltitude é\n";
        let mut le = vec![0xff, 0xfe];
        let mut be = vec![0xfe, 0xff];
        for unit in text.encode_utf16() {
            le.extend(unit.to_le_bytes());
            be.extend(unit.to_be_bytes());
        }
        assert_eq!(decode(&le), text);
        assert_eq!(decode(&be), text);
        // An odd byte at the end, and half a surrogate pair, are each one replaced character.
        le.push(b'x');
        assert_eq!(decode(&le), format!("{text}\u{fffd}"));
        assert_eq!(decode(&[0xff, 0xfe, 0x3d, 0xd8]), "\u{fffd}");
        assert_eq!(decode(b"\xef\xbb\xbfa\xff"), "a\u{fffd}");
    }

    #[test]
    fn a_layout_no_reader_knows_is_refused_naming_those_that_exist() {
        let error = read(b"Time,Pressure\n0,101325\n").unwrap_err();
        let LogError::Unrecognized { message } = &error else {
            panic!("refused for another reason: {error:?}");
        };
        for format in FORMATS {
            assert!(message.contains(format.name()), "{message}");
        }
    }
}
