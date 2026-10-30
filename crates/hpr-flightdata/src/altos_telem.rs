//! AltOS radio telemetry, as AltosUI saves the packets a TeleDongle hears to a `.telem` file:
//! one `TELEM` line a packet, its bytes in hexadecimal.
//!
//! An Altus Metrum flight computer sends a 32-byte packet over its radio several times a second;
//! the ground station's receiver, a TeleDongle, writes each packet it hears to its serial line as
//! a text line, `TELEM` and then, in hexadecimal, a length byte, the packet, two bytes its radio
//! adds, the signal strength (RSSI) and a link-quality byte whose top bit says the packet passed
//! the radio's CRC, and a checksum (*AltOS Telemetry*, Bdale Garbee and Keith Packard,
//! v1.9.22.2, 2025, §4 "TeleDongle serial packet format", p. 10). AltosUI saves every packet it
//! receives (*The Altus Metrum System*, v1.9.22.2, 2025, §12.1, p. 27); the manual doesn't show
//! the file, and both sample files hold `TELEM` lines only, failed packets among them, one a line.
//! The document is published under CC BY-SA 3.0; this reader restates its facts, not its tables.
//!
//! **Recognition.** The first line that isn't blank is `TELEM`, a space, and an even number of
//! hexadecimal digits; no other layout HPR Sim reads opens so.
//!
//! **A line.** Every line that isn't blank must be such a packet, 36 bytes: the length byte 34
//! (the 32-byte packet and the two bytes the radio adds), the packet, the RSSI, the link-quality
//! byte and the checksum, which is 0x5A plus every byte after the length byte up to, not
//! including, the checksum, modulo 256. The document's table puts its byte positions one place early, and its formula taken as
//! written fails its own example; summing from the byte after the length through the
//! link-quality byte passes the example and every line of both sample files. A line that isn't a
//! packet, has another length, or fails its checksum refuses the file ([`LogError::Syntax`]):
//! the TeleDongle computed the checksum over its own line, so a failure is damage to the file. A
//! packet whose link-quality byte says it failed the radio's CRC is left out and counted in a
//! note: its bytes are noise (13.6% and 4.1% of the two sample files' packets).
//!
//! **A packet.** The first five bytes are the board's serial number (16 bits), its clock in
//! ticks of 10 ms (16 bits) and the packet's type, then 27 bytes the type lays out (§2.1, p. 2).
//! The document doesn't state the byte order; read little-endian, its own example's position is
//! 45.47° N, 122.74° W, and big-endian it isn't a latitude, so little-endian it is. Every packet
//! must name the same serial number, or the file is refused: two boards' packets in one file
//! would make two flights of one. The RSSI is a signed byte, in dBm as `RSSI / 2 − 74` (§4); the
//! document doesn't say signed, but the manual's receivers hear down to −111 dBm (§12.1, p. 27),
//! which needs it, and read unsigned the sample files' signals would be stronger than the
//! transmitter. The link-quality byte's other seven bits are kept as written.
//!
//! **The clock.** The tick turns over every 655.36 s. Each packet's tick is placed on the turn
//! nearest the latest tick so far, so a GPS packet, which carries the earlier tick of its fix, is
//! placed behind, not a turn ahead. The record's clock is that tick in seconds, counted from the
//! turn of the first packet heard, not from boost: how long the board had run before is unknown.
//! A board switched off and on again restarts its tick, so a packet whose tick falls behind the
//! last one of its own type starts a new power-on (within one, each type's ticks never fall
//! back). The record holds one power-on: the one whose sensor packets leave the pad (a state
//! from `boost` to `main`), or, where none does, the last; packets of the others are left out and
//! counted in a note, and a file in which two power-ons leave the pad is refused. The tick alone
//! can't tell everything apart: a silence of 327.68 s to 655.36 s, half a turn to a whole one,
//! reads as a step back, and so as a power-on; a longer one reads as a turn less than it was;
//! and a restart whose new tick lands ahead of the last, as when a board had run for more than
//! half a turn before it, reads as a silence, the two power-ons one record.
//!
//! **What is read.** The two sample files hold five packet types, and only those are decoded;
//! the document lays out eleven more, for other boards, and a packet of one is counted in a note
//! and not read: with no sample to check it, a misread unit wouldn't show (the document's own
//! pressure unit is wrong, below). Each field is kept in the unit below where a second check bears
//! it out, and as written otherwise:
//!
//! | Packet | Field | Channel | As read |
//! |---|---|---|---|
//! | `0x0A`, TeleMetrum v2 and newer, sensors | `state` | `state`, [`Quantity::Number`] | as written; each change an [`Event`] |
//! | | `pres` | `pressure`, [`Quantity::Pressure`] | Pa: the document says tenths of a pascal, but the files hold pascals (below) |
//! | | `temp` | `temperature`, [`Quantity::Temperature`] | hundredths of a °C, the document's; K here |
//! | | `acceleration` | `acceleration`, [`Quantity::Acceleration`] | sixteenths of a m/s², the document's: the board's own estimate along its axis |
//! | | `speed` | `speed`, [`Quantity::Speed`] | sixteenths of a m/s, the document's: the board's own estimate |
//! | | `height` | `height`, [`Quantity::AltitudeAboveGround`] | m above the pad, the document's: the board's own estimate |
//! | | `accel`, `v_batt`, `sense_d`, `sense_m` | the same names, [`Quantity::Unrecognized`] | as written: the document gives them as the converters' counts and no conversion |
//! | `0x0B`, TeleMetrum v2 and newer, calibration | `ground_pres` | `ground_pressure`, [`Quantity::Pressure`] | Pa, as `pres` |
//! | | `ground_accel`, `accel_plus_g`, `accel_minus_g` | the same names, [`Quantity::Unrecognized`] | as written: counts |
//! | `0x05`, GPS position | flags | `nsat` (its low four bits) and `gps_flags` (the byte), [`Quantity::Number`] | as written |
//! | | latitude, longitude | `latitude`, `longitude` | degrees, from ten-millionths; a gap where the flags' valid bit is clear |
//! | | altitude | `gps_altitude`, [`Quantity::GnssAltitude`] | m; a gap where the valid bit is clear |
//! | | year … second, `pdop`, `hdop`, `vdop`, mode | the same names, [`Quantity::Number`] | as written: the year is two digits, the dilutions five times their value in the document |
//! | | ground speed, climb rate, course | `ground_speed`, `climb_rate`, `course`, [`Quantity::Unrecognized`] | as written: the document's cm/s and "/2" are unchecked |
//! | `0x06`, GPS satellites | the count, and 12 pairs of a satellite's id and its signal | `sat_channels`, `svid_01` … `svid_12`, `c_n_01` … `c_n_12`, [`Quantity::Number`] | as written |
//! | `0x04`, configuration | flight number, settings, call sign, firmware | the record's [`FlightLog::flight_number`] and [`FlightLog::firmware`]; each configuration in a note | as written |
//! | every packet | RSSI, link quality | `rssi` (dBm) and `lqi`, [`Quantity::Number`] | dBm by the document's formula; the link quality as written |
//!
//! *Pressure in pascals, not tenths.* The document gives `pres` as "Pa * 10". Read so, both sample
//! files' pads would be more than 15 km up; read as pascals they are at the height above sea
//! level the export beside one of them states, and that export's own pressures, in pascals, equal
//! the telemetry's at every tick where the export holds a fresh sample (the survey's report,
//! `cargo xtask logs`). So `pres` is read as pascals where the
//! pressure on the pad, the median of the packets in the pad state or else the first packet's,
//! lies between 10 and 120 kPa read so, as every launch site's does; otherwise the column is
//! kept as written, [`Quantity::Unrecognized`], with a note.
//!
//! *The flight's states.* The document names no state's number. AltosUI's exports write the
//! number and the name side by side, and in every one of the sample exports 2 is `pad`, 3
//! `boost`, 4 `fast`, 5 `coast`, 6 `drogue`, 7 `main` and 8 `landed`; the events take those
//! names, and another number is `state N`. The board's estimates, `acceleration`, `speed` and
//! `height`, are checked in the survey against each other and against the export.
//!
//! **The primary track.** [`FlightLog::altitude_m`] is `height` on the sensor packets' clock:
//! the board's own estimate of its height above the pad, which the telemetry sends in place of
//! a barometric altitude and which may lean on its accelerometer as its `speed` does; the document
//! doesn't say how it is formed. [`FlightLog::temperature_k`] is the temperature; the vertical speed is absent, as the board's
//! `speed` leans on its accelerometer (as for [AltOS's CSV](crate::altos)). Every other field is
//! a channel on its own packets' clock.
//!
//! An invented line, a configuration packet: serial 1234 (`d204`), tick 256 (`0001`), type
//! `04`, then device type 11, flight 3, configuration 1.27, no apogee delay, main at 250 m, a log
//! of 2,048 kB, call sign `N0CALL` and firmware `1.9.22`; the RSSI byte `f4`, −12, is −80 dBm,
//! and `a8` sets the CRC bit, with a link quality of 40:
//!
//! ```text
//! TELEM 22d2040001040b0300011b0000fa0000084e3043414c4c0000312e392e32320000f4a8c1
//! ```

use std::collections::BTreeMap;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};

/// The format's name in errors.
const FORMAT: &str = "AltOS telemetry";

/// The word every line opens with.
const PREFIX: &str = "TELEM";

/// A line's length byte: the 32-byte packet and the radio's two bytes.
const LENGTH: u8 = 34;

/// A line's bytes: the length byte, then [`LENGTH`] bytes, then the checksum.
const LINE_BYTES: usize = LENGTH as usize + 2;

/// What the checksum adds to the sum of the bytes it covers (§4).
const CHECKSUM_BASE: u8 = 0x5a;

/// The link-quality byte's bit that says the packet passed the radio's CRC (§4).
const CRC_OK: u8 = 0x80;

/// The tick's turn: 2¹⁶ ticks.
const TURN: i64 = 1 << 16;

/// The ticks in a second (§2.1): a time is the tick divided by it, correctly rounded, where
/// multiplying by 0.01 would not be (510 × 0.01 is 5.100 000 000 000 000 5).
const TICKS_PER_S: f64 = 100.0;

/// The pressures a launch site's pad can have, Pa.
const PAD_PRESSURE_PA: std::ops::RangeInclusive<f64> = 10_000.0..=120_000.0;

/// The flight's states by number, as AltosUI's exports name them.
pub(crate) const STATES: [(u8, &str); 7] = [
    (2, "pad"),
    (3, "boost"),
    (4, "fast"),
    (5, "coast"),
    (6, "drogue"),
    (7, "main"),
    (8, "landed"),
];

/// The pad's state number.
const PAD: u8 = 2;

/// The states of a flight, boost to main: a power-on in one of them has left the pad. `landed`
/// isn't among them, so a power-on heard only after landing, such as one that starts after a
/// silence, isn't taken for a second flight.
const FLOWN: std::ops::RangeInclusive<u8> = 3..=7;

/// How many distinct configurations a note quotes.
const QUOTED_CONFIGURATIONS: usize = 8;

/// The packet types the reader decodes.
const SENSOR: u8 = 0x0a;
const CALIBRATION: u8 = 0x0b;
const CONFIGURATION: u8 = 0x04;
const LOCATION: u8 = 0x05;
const SATELLITES: u8 = 0x06;

/// The satellite packet's twelve entries' channels: each satellite's id and its signal.
const SVID: [&str; 12] = [
    "svid_01", "svid_02", "svid_03", "svid_04", "svid_05", "svid_06", "svid_07", "svid_08",
    "svid_09", "svid_10", "svid_11", "svid_12",
];
const C_N: [&str; 12] = [
    "c_n_01", "c_n_02", "c_n_03", "c_n_04", "c_n_05", "c_n_06", "c_n_07", "c_n_08", "c_n_09",
    "c_n_10", "c_n_11", "c_n_12",
];

/// The types the document lays out that the reader doesn't decode, with the document's names.
const NOT_READ: [(u8, &str); 11] = [
    (0x01, "TeleMetrum v1 sensors"),
    (0x02, "TeleMini v1 sensors"),
    (0x03, "TeleNano sensors"),
    (0x07, "companion"),
    (0x08, "TeleMega sensors"),
    (0x09, "TeleMega data"),
    (0x11, "TeleMini v3 sensors"),
    (0x12, "TeleMega sensors (BMX160)"),
    (0x13, "TeleMega sensors (MPU6000 and MMC5983)"),
    (0x14, "TeleMega sensors (BMI088 and MMC5983)"),
    (0x15, "TeleMega data (30 V)"),
];

/// Whether the text is AltOS telemetry: its first line that isn't blank is `TELEM`, a space, and
/// an even number of hexadecimal digits.
pub fn recognizes(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .and_then(hex_of)
        .is_some()
}

/// A line's hexadecimal digits, if it is `TELEM`, whitespace and an even number of them.
fn hex_of(line: &str) -> Option<&str> {
    let rest = line.strip_prefix(PREFIX)?;
    let digits = rest.trim_start();
    (digits.len() < rest.len()
        && !digits.is_empty()
        && digits.len() % 2 == 0
        && digits.bytes().all(|b| b.is_ascii_hexdigit()))
    .then_some(digits)
}

/// One packet that passed the radio's CRC.
struct Packet {
    line: usize,
    tick: u16,
    kind: u8,
    bytes: [u8; 32],
    rssi_dbm: f64,
    lqi: u8,
}

impl Packet {
    fn u8(&self, at: usize) -> f64 {
        f64::from(self.bytes[at])
    }
    fn i16(&self, at: usize) -> f64 {
        f64::from(i16::from_le_bytes([self.bytes[at], self.bytes[at + 1]]))
    }
    fn u16(&self, at: usize) -> u16 {
        u16::from_le_bytes([self.bytes[at], self.bytes[at + 1]])
    }
    fn i32(&self, at: usize) -> f64 {
        f64::from(i32::from_le_bytes([
            self.bytes[at],
            self.bytes[at + 1],
            self.bytes[at + 2],
            self.bytes[at + 3],
        ]))
    }
    /// A text field: its bytes up to the first NUL, a byte that isn't printable ASCII as `?`.
    fn text(&self, at: usize, len: usize) -> String {
        self.bytes[at..at + len]
            .iter()
            .take_while(|b| **b != 0)
            .map(|b| {
                if b.is_ascii_graphic() || *b == b' ' {
                    char::from(*b)
                } else {
                    '?'
                }
            })
            .collect::<String>()
            .trim()
            .to_owned()
    }
}

/// The channels being built, by name, in the order first met.
#[derive(Default)]
struct Channels {
    order: Vec<(String, Quantity)>,
    data: BTreeMap<String, (Vec<f64>, Vec<f64>)>,
}

impl Channels {
    fn push(&mut self, name: &str, quantity: Quantity, time_s: f64, value: f64) {
        // Names are allocated on a channel's first sample only.
        if let Some((times, values)) = self.data.get_mut(name) {
            times.push(time_s);
            values.push(value);
            return;
        }
        self.order.push((name.to_owned(), quantity));
        self.data
            .insert(name.to_owned(), (vec![time_s], vec![value]));
    }

    fn values(&self, name: &str) -> Option<&[f64]> {
        self.data.get(name).map(|(_, values)| values.as_slice())
    }

    fn into_channels(mut self) -> Vec<Channel> {
        self.order
            .into_iter()
            .filter_map(|(column, quantity)| {
                let (time_s, values) = self.data.remove(&column)?;
                Some(Channel {
                    column,
                    quantity,
                    time_s,
                    values,
                })
            })
            .collect()
    }
}

/// Reads AltOS telemetry, as AltosUI saves it to a `.telem` file.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes(text) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "the first line that isn't blank isn't `{PREFIX}` and a packet in hexadecimal"
            ),
        });
    }
    let (all, crc_failed) = packets(text)?;
    let Some(first) = all.first() else {
        return Err(LogError::NoData { format: FORMAT });
    };
    let serial = first.u16(0);
    if let Some(other) = all.iter().find(|packet| packet.u16(0) != serial) {
        return Err(syntax(
            other.line,
            format!(
                "a packet from serial number {}, where the file's first is from {serial}: two \
                 boards' telemetry in one file",
                other.u16(0)
            ),
        ));
    }

    // Power-ons: within one, each type's ticks never fall back.
    let mut starts = vec![0usize];
    let mut last: BTreeMap<u8, u16> = BTreeMap::new();
    for (index, packet) in all.iter().enumerate() {
        if let Some(previous) = last.get(&packet.kind)
            && step(*previous, packet.tick) < 0
        {
            starts.push(index);
            last.clear();
        }
        last.insert(packet.kind, packet.tick);
    }
    let session = |k: usize| &all[starts[k]..starts.get(k + 1).copied().unwrap_or(all.len())];
    let flown: Vec<usize> = (0..starts.len())
        .filter(|&k| {
            session(k)
                .iter()
                .any(|packet| packet.kind == SENSOR && FLOWN.contains(&packet.bytes[5]))
        })
        .collect();
    let chosen = match flown.as_slice() {
        [] => starts.len() - 1,
        [only] => *only,
        [first, second, ..] => {
            return Err(syntax(
                session(*second)[0].line,
                format!(
                    "the board left the pad in two power-ons, from line {} and from this one: \
                     which is the flight is unclear",
                    session(*first)[0].line
                ),
            ));
        }
    };
    let packets = session(chosen);

    // The clock: each tick on the turn nearest the latest so far.
    let mut latest: Option<i64> = None;
    let times: Vec<f64> = packets
        .iter()
        .map(|packet| {
            #[allow(
                clippy::cast_possible_truncation,
                reason = "a remainder of a 65,536-tick turn, so the cast is exact"
            )]
            let unwound = match latest {
                None => i64::from(packet.tick),
                Some(latest) => {
                    latest + i64::from(step(latest.rem_euclid(TURN) as u16, packet.tick))
                }
            };
            latest = Some(latest.map_or(unwound, |latest| latest.max(unwound)));
            unwound as f64 / TICKS_PER_S
        })
        .collect();

    let pressure_in_pa = pad_pressure(packets).is_some_and(|pa| PAD_PRESSURE_PA.contains(&pa));
    let pressure = |name: &'static str, raw: &'static str| {
        if pressure_in_pa {
            (name, Quantity::Pressure)
        } else {
            (raw, Quantity::Unrecognized)
        }
    };

    let mut channels = Channels::default();
    let mut events: Vec<Event> = Vec::new();
    let mut state: Option<u8> = None;
    let mut track_time: Vec<f64> = Vec::new();
    let mut track_height: Vec<f64> = Vec::new();
    let mut track_temperature: Vec<f64> = Vec::new();
    let mut repeated_sensor_ticks = 0usize;
    let mut not_read: BTreeMap<u8, usize> = BTreeMap::new();
    let mut configurations: Vec<String> = Vec::new();
    let mut more_configurations = 0usize;
    let mut flight_number: Option<u16> = None;
    let mut flight_numbers_differ = false;
    let mut firmware: Option<String> = None;
    let mut firmware_differs = false;
    let mut no_fix = 0usize;
    for (packet, &time) in packets.iter().zip(&times) {
        let read = matches!(
            packet.kind,
            SENSOR | CALIBRATION | CONFIGURATION | LOCATION | SATELLITES
        );
        if !read {
            *not_read.entry(packet.kind).or_default() += 1;
            continue;
        }
        channels.push("rssi", Quantity::Number, time, packet.rssi_dbm);
        channels.push("lqi", Quantity::Number, time, f64::from(packet.lqi));
        match packet.kind {
            SENSOR => {
                let now = packet.bytes[5];
                if state != Some(now) {
                    events.push(Event {
                        time_s: time,
                        name: state_name(now),
                    });
                    state = Some(now);
                }
                channels.push("state", Quantity::Number, time, f64::from(now));
                channels.push("accel", Quantity::Unrecognized, time, packet.i16(6));
                let (name, quantity) = pressure("pressure", "pres");
                channels.push(name, quantity, time, packet.i32(8));
                let kelvin = packet.i16(12) / 100.0 + 273.15;
                channels.push("temperature", Quantity::Temperature, time, kelvin);
                let acceleration = packet.i16(14) / 16.0;
                channels.push("acceleration", Quantity::Acceleration, time, acceleration);
                channels.push("speed", Quantity::Speed, time, packet.i16(16) / 16.0);
                let height = packet.i16(18);
                channels.push("height", Quantity::AltitudeAboveGround, time, height);
                channels.push("v_batt", Quantity::Unrecognized, time, packet.i16(20));
                channels.push("sense_d", Quantity::Unrecognized, time, packet.i16(22));
                channels.push("sense_m", Quantity::Unrecognized, time, packet.i16(24));
                if track_time.last().is_some_and(|last| time <= *last) {
                    repeated_sensor_ticks += 1;
                } else {
                    track_time.push(time);
                    track_height.push(height);
                    track_temperature.push(kelvin);
                }
            }
            CALIBRATION => {
                let (name, quantity) = pressure("ground_pressure", "ground_pres");
                channels.push(name, quantity, time, packet.i32(8));
                channels.push("ground_accel", Quantity::Unrecognized, time, packet.i16(12));
                channels.push("accel_plus_g", Quantity::Unrecognized, time, packet.i16(14));
                channels.push(
                    "accel_minus_g",
                    Quantity::Unrecognized,
                    time,
                    packet.i16(16),
                );
            }
            CONFIGURATION => {
                let flight = packet.u16(6);
                if flight_number.is_some_and(|number| number != flight) {
                    flight_numbers_differ = true;
                }
                flight_number.get_or_insert(flight);
                let version = packet.text(24, 8);
                if !version.is_empty() {
                    if firmware.as_ref().is_some_and(|first| *first != version) {
                        firmware_differs = true;
                    }
                    firmware.get_or_insert_with(|| version.clone());
                }
                let configuration = format!(
                    "device type {}, flight {flight}, configuration {}.{}, apogee delay {} s, \
                     main at {} m, log of {} kB, call sign {:?}, firmware {:?}",
                    packet.bytes[5],
                    packet.bytes[8],
                    packet.bytes[9],
                    packet.u16(10),
                    packet.u16(12),
                    packet.u16(14),
                    packet.text(16, 8),
                    version
                );
                if !configurations.contains(&configuration) {
                    if configurations.len() < QUOTED_CONFIGURATIONS {
                        configurations.push(configuration);
                    } else {
                        more_configurations += 1;
                    }
                }
            }
            LOCATION => {
                let flags = packet.bytes[5];
                channels.push("nsat", Quantity::Number, time, f64::from(flags & 0x0f));
                channels.push("gps_flags", Quantity::Number, time, f64::from(flags));
                let valid = flags & 0x10 != 0;
                if !valid {
                    no_fix += 1;
                }
                let fix = |value: f64| if valid { value } else { f64::NAN };
                channels.push(
                    "latitude",
                    Quantity::Latitude,
                    time,
                    fix(packet.i32(8) / 1e7),
                );
                channels.push(
                    "longitude",
                    Quantity::Longitude,
                    time,
                    fix(packet.i32(12) / 1e7),
                );
                channels.push(
                    "gps_altitude",
                    Quantity::GnssAltitude,
                    time,
                    fix(packet.i16(6)),
                );
                for (k, name) in ["year", "month", "day", "hour", "minute", "second"]
                    .into_iter()
                    .enumerate()
                {
                    channels.push(name, Quantity::Number, time, packet.u8(16 + k));
                }
                for (k, name) in ["pdop", "hdop", "vdop", "mode"].into_iter().enumerate() {
                    channels.push(name, Quantity::Number, time, packet.u8(22 + k));
                }
                let ground_speed = f64::from(packet.u16(26));
                channels.push("ground_speed", Quantity::Unrecognized, time, ground_speed);
                channels.push("climb_rate", Quantity::Unrecognized, time, packet.i16(28));
                channels.push("course", Quantity::Unrecognized, time, packet.u8(30));
            }
            SATELLITES => {
                channels.push("sat_channels", Quantity::Number, time, packet.u8(5));
                for (k, (svid, c_n)) in SVID.iter().zip(C_N).enumerate() {
                    channels.push(svid, Quantity::Number, time, packet.u8(6 + 2 * k));
                    channels.push(c_n, Quantity::Number, time, packet.u8(7 + 2 * k));
                }
            }
            _ => {}
        }
    }
    if channels.data.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }

    let mut notes = Vec::new();
    notes.push(format!(
        "{} packets heard: {crc_failed} failed the radio's CRC and are left out",
        all.len() + crc_failed
    ));
    if starts.len() > 1 {
        let others: usize = (0..starts.len())
            .filter(|&k| k != chosen)
            .map(|k| session(k).len())
            .sum();
        notes.push(format!(
            "the board's tick starts again {} times, a power-on each. The record holds power-on \
             {} of {} ({}), and {others} packets of the others are left out",
            starts.len() - 1,
            chosen + 1,
            starts.len(),
            if flown.is_empty() {
                "the last, as none leaves the pad"
            } else {
                "the one that leaves the pad"
            }
        ));
    }
    notes.push(format!(
        "the clock is the board's tick, 10 ms, from the turn of the first packet heard. Each \
         tick is on the turn nearest the latest so far. The clock's first packet is at {:.2} s",
        times[0]
    ));
    match pad_pressure(packets) {
        Some(pa) if pressure_in_pa => notes.push(format!(
            "`pres` read in pascals, not the document's tenths of a pascal: the pad's pressure, \
             {pa:.0}, lies between 10 and 120 kPa read so"
        )),
        Some(pa) => notes.push(format!(
            "`pres` kept as written. The pad's pressure, {pa:.0}, is no launch site's \
             pressure in pascals. The files read contradict the document's tenths of a pascal"
        )),
        None => {}
    }
    if channels.values("accel").is_some() {
        notes.push(
            "`accel`, `v_batt`, `sense_d` and `sense_m` kept as written: the converters' counts, \
             which the document gives no conversion for"
                .to_owned(),
        );
        notes.push(
            "`acceleration`, `speed` and `height` are the board's own estimates, read in the \
             document's units. The units are sixteenths of a m/s² and of a m/s, and meters above \
             the pad"
                .to_owned(),
        );
    }
    if channels.values("ground_accel").is_some() {
        notes.push(
            "`ground_accel`, `accel_plus_g` and `accel_minus_g` kept as written: counts".to_owned(),
        );
    }
    if channels.values("nsat").is_some() {
        notes.push(
            "GPS: `pdop`, `hdop` and `vdop` kept as written, five times the dilution in the \
             document. The date and time are as written, the year two digits. `ground_speed`, \
             `climb_rate` and `course` kept as written, their units unchecked"
                .to_owned(),
        );
    }
    if no_fix > 0 {
        notes.push(format!(
            "{no_fix} GPS packets with the flags' valid bit clear: their position and altitude \
             read as gaps"
        ));
    }
    if repeated_sensor_ticks > 0 {
        notes.push(format!(
            "{repeated_sensor_ticks} sensor packets at a tick no later than the one before: kept \
             as channels, left off the primary track"
        ));
    }
    if !configurations.is_empty() {
        let mut quoted = configurations.join(" | ");
        if more_configurations > 0 {
            quoted.push_str(&format!(" | and {more_configurations} more"));
        }
        notes.push(format!("the configuration, as sent: {quoted}"));
    }
    if flight_numbers_differ {
        notes.push(
            "the configuration packets name more than one flight number: the record's is the \
             first"
                .to_owned(),
        );
    }
    if firmware_differs {
        notes.push(
            "the configuration packets name more than one firmware version: the record's is the \
             first"
                .to_owned(),
        );
    }
    for (kind, count) in &not_read {
        notes.push(match NOT_READ.iter().find(|(known, _)| known == kind) {
            Some((_, name)) => format!(
                "{count} packets of type 0x{kind:02X} ({name}) not read: no sample file holds \
                 one to check the document against"
            ),
            None => format!(
                "{count} packets of type 0x{kind:02X} not read: a type the document doesn't lay \
                 out"
            ),
        });
    }

    let has_sensors = !track_time.is_empty();
    Ok(FlightLog {
        format: LogFormat::AltosTelemetry,
        logger: if has_sensors {
            "AltOS TeleMetrum v2 or newer".to_owned()
        } else {
            "AltOS".to_owned()
        },
        serial_number: Some(serial.to_string()),
        firmware,
        flight_number: flight_number.map(u32::from),
        stated: Stated::default(),
        time_s: track_time,
        altitude_m: track_height,
        vertical_speed_m_s: None,
        temperature_k: has_sensors.then_some(track_temperature),
        battery_v: None,
        channels: channels.into_channels(),
        events,
        notes,
    })
}

/// Every line's packet that passed the radio's CRC, and how many didn't; a refusal at a line
/// that isn't a packet or fails its checksum.
fn packets(text: &str) -> Result<(Vec<Packet>, usize), LogError> {
    let mut packets = Vec::new();
    let mut crc_failed = 0;
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let digits = hex_of(line).ok_or_else(|| {
            syntax(
                number,
                format!(
                    "a line that isn't `{PREFIX}` and a packet in hexadecimal: {:?}",
                    cut(line)
                ),
            )
        })?;
        if digits.len() != 2 * LINE_BYTES {
            return Err(syntax(
                number,
                format!(
                    "a line of {} bytes, where a TeleDongle's line is {LINE_BYTES}",
                    digits.len() / 2
                ),
            ));
        }
        let mut bytes = [0u8; LINE_BYTES];
        for (k, byte) in bytes.iter_mut().enumerate() {
            // Checked as hexadecimal digits above, two a byte.
            *byte = u8::from_str_radix(&digits[2 * k..2 * k + 2], 16).unwrap_or(0);
        }
        if bytes[0] != LENGTH {
            return Err(syntax(
                number,
                format!(
                    "a length byte of {}, where a TeleDongle's line holds {LENGTH}: a 32-byte \
                     packet and the radio's two bytes",
                    bytes[0]
                ),
            ));
        }
        let sum = bytes[1..LINE_BYTES - 1]
            .iter()
            .fold(CHECKSUM_BASE, |sum, byte| sum.wrapping_add(*byte));
        if sum != bytes[LINE_BYTES - 1] {
            return Err(syntax(
                number,
                format!(
                    "the checksum is 0x{:02X} where the line's bytes sum to 0x{sum:02X}: the line \
                     is damaged",
                    bytes[LINE_BYTES - 1]
                ),
            ));
        }
        let lqi_byte = bytes[34];
        if lqi_byte & CRC_OK == 0 {
            crc_failed += 1;
            continue;
        }
        let mut packet = [0u8; 32];
        packet.copy_from_slice(&bytes[1..33]);
        packets.push(Packet {
            line: number,
            tick: u16::from_le_bytes([packet[2], packet[3]]),
            kind: packet[4],
            bytes: packet,
            rssi_dbm: f64::from(i8::from_le_bytes([bytes[33]])) / 2.0 - 74.0,
            lqi: lqi_byte & !CRC_OK,
        });
    }
    Ok((packets, crc_failed))
}

/// The ticks from `from` to `to`, on the nearest turn: from −32,768 to 32,767.
fn step(from: u16, to: u16) -> i16 {
    to.wrapping_sub(from) as i16
}

/// The pad's pressure as written: the median of the sensor packets in the pad state, or else the
/// first sensor packet's; `None` without one.
fn pad_pressure(packets: &[Packet]) -> Option<f64> {
    let sensors = || packets.iter().filter(|packet| packet.kind == SENSOR);
    let mut pad: Vec<f64> = sensors()
        .filter(|packet| packet.bytes[5] == PAD)
        .map(|packet| packet.i32(8))
        .collect();
    if pad.is_empty() {
        return sensors().next().map(|packet| packet.i32(8));
    }
    pad.sort_by(f64::total_cmp);
    let middle = pad.len() / 2;
    Some(if pad.len() % 2 == 1 {
        pad[middle]
    } else {
        (pad[middle - 1] + pad[middle]) / 2.0
    })
}

/// A state's name, as AltosUI's exports write it, or `state N`.
pub(crate) fn state_name(number: u8) -> String {
    STATES
        .iter()
        .find(|(known, _)| *known == number)
        .map_or_else(|| format!("state {number}"), |(_, name)| (*name).to_owned())
}

/// A line cut to 80 characters, for an error.
fn cut(text: &str) -> String {
    text.chars().take(80).collect()
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A packet's line: `TELEM`, the length, the packet (serial, tick, type, then `data`, padded
    /// with zeros to 32 bytes), the RSSI byte, the link-quality byte and the checksum.
    fn line_with(serial: u16, tick: u16, kind: u8, data: &[u8], rssi: u8, lqi: u8) -> String {
        let mut bytes = vec![LENGTH];
        bytes.extend(serial.to_le_bytes());
        bytes.extend(tick.to_le_bytes());
        bytes.push(kind);
        bytes.extend(data);
        bytes.resize(33, 0);
        bytes.extend([rssi, lqi]);
        let sum = bytes[1..]
            .iter()
            .fold(CHECKSUM_BASE, |sum, byte| sum.wrapping_add(*byte));
        bytes.push(sum);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        format!("TELEM {hex}")
    }

    /// A packet that passed the radio's CRC, from serial 1234, its RSSI −80 dBm.
    fn line(tick: u16, kind: u8, data: &[u8]) -> String {
        line_with(1234, tick, kind, data, (-12i8) as u8, 0x80 | 40)
    }

    /// A TeleMetrum v2 sensor packet: state, pressure (Pa as written), temperature (hundredths of
    /// a °C), acceleration and speed (sixteenths), height (m).
    fn sensor(
        tick: u16,
        state: u8,
        pres: i32,
        temp: i16,
        acc: i16,
        speed: i16,
        height: i16,
    ) -> String {
        let mut data = vec![state];
        data.extend(100i16.to_le_bytes());
        data.extend(pres.to_le_bytes());
        data.extend(temp.to_le_bytes());
        data.extend(acc.to_le_bytes());
        data.extend(speed.to_le_bytes());
        data.extend(height.to_le_bytes());
        data.extend([1i16, 2, 3].iter().flat_map(|v| v.to_le_bytes()));
        line(tick, SENSOR, &data)
    }

    fn file(lines: &[String]) -> String {
        lines.join("\n") + "\n"
    }

    fn refusal(text: &str) -> String {
        match read(text) {
            Err(LogError::Syntax { message, .. }) => message,
            other => panic!("not a syntax refusal: {other:?}"),
        }
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> Option<&'a Channel> {
        log.channels.iter().find(|c| c.column == column)
    }

    #[test]
    fn the_layout_is_recognized_by_its_first_line() {
        assert!(recognizes(&file(&[sensor(100, 2, 98_000, 2_000, 0, 0, 0)])));
        assert!(recognizes("\n  TELEM 22ab\n"));
        assert!(!recognizes("TELEM 22a\n"), "an odd count of digits");
        assert!(!recognizes("TELEM\n"));
        assert!(!recognizes("TELEMX 22ab\n"));
        assert!(!recognizes("TELEM 22zz\n"));
        assert!(!recognizes("#version,serial,flight\n"));
    }

    /// The document's own example line passes the checksum as the reader sums it, from the byte
    /// after the length through the link-quality byte, and fails it summed from the length byte.
    #[test]
    fn the_checksum_covers_the_bytes_after_the_length() {
        let good = sensor(100, 2, 98_000, 2_000, 0, 0, 0);
        assert!(read(&file(std::slice::from_ref(&good))).is_ok());
        // One byte of the packet changed, the checksum not: refused, naming the line.
        let mut damaged = good.clone().into_bytes();
        damaged[20] = if damaged[20] == b'0' { b'1' } else { b'0' };
        let damaged = String::from_utf8(damaged).unwrap();
        let text = file(&[good.clone(), damaged]);
        assert!(refusal(&text).contains("the line is damaged"));
        match read(&text) {
            Err(LogError::Syntax { line, .. }) => assert_eq!(line, 2),
            other => panic!("{other:?}"),
        }
        // A checksum computed from the length byte instead is off by 0x22.
        let hex = good.strip_prefix("TELEM ").unwrap();
        let mut bytes: Vec<u8> = (0..hex.len() / 2)
            .map(|k| u8::from_str_radix(&hex[2 * k..2 * k + 2], 16).unwrap())
            .collect();
        let last = bytes.len() - 1;
        bytes[last] = bytes[last].wrapping_add(LENGTH);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        assert!(refusal(&format!("TELEM {hex}\n")).contains("damaged"));
    }

    #[test]
    fn a_line_that_isnt_a_packet_refuses_the_file() {
        let good = sensor(100, 2, 98_000, 2_000, 0, 0, 0);
        assert!(refusal(&file(&[good.clone(), "TELEM xyz".to_owned()])).contains("isn't `TELEM`"));
        assert!(
            refusal(&file(&[good.clone(), "# a comment".to_owned()])).contains("isn't `TELEM`")
        );
        // 35 bytes: the checksum missing.
        let short = &good[..good.len() - 2];
        assert!(refusal(&file(&[short.to_owned()])).contains("of 35 bytes"));
        // A length byte of 33 in a 36-byte line.
        let other_length = good.replacen("TELEM 22", "TELEM 21", 1);
        assert!(refusal(&file(&[other_length])).contains("length byte of 33"));
    }

    /// A packet the radio flags as failing its CRC is left out and counted, its bytes unread.
    #[test]
    fn a_packet_failing_the_radios_crc_is_counted_not_read() {
        let noise = line_with(9999, 7, 0xee, &[0xff; 27], 0x90, 0x12);
        let text = file(&[
            sensor(100, 2, 98_000, 2_000, 0, 0, 0),
            noise,
            sensor(110, 2, 98_010, 2_000, 0, 0, 1),
        ]);
        let log = read(&text).unwrap();
        assert_eq!(log.time_s, [1.0, 1.1]);
        assert!(log.notes[0].starts_with("3 packets heard: 1 failed the radio's CRC"));
        assert_eq!(log.serial_number.as_deref(), Some("1234"));
    }

    #[test]
    fn two_boards_in_one_file_are_refused() {
        let other = line_with(4321, 105, SENSOR, &[2], (-12i8) as u8, 0x80);
        let text = file(&[sensor(100, 2, 98_000, 2_000, 0, 0, 0), other]);
        assert!(refusal(&text).contains("two boards' telemetry"));
    }

    /// The units: pressure in pascals, temperature from hundredths of a °C to K, acceleration and
    /// speed from sixteenths, height in meters; the counts kept as written; the states named.
    #[test]
    fn a_sensor_packet_reads_in_the_units_borne_out() {
        let text = file(&[
            sensor(100, 2, 98_000, 2_150, -8, 0, 0),
            sensor(110, 3, 97_500, 2_160, 1_600, 320, 4),
            sensor(120, 9, 97_000, 2_170, -160, 640, 12),
        ]);
        let log = read(&text).unwrap();
        let values = |column: &str| channel(&log, column).unwrap().values.clone();
        assert_eq!(values("pressure"), [98_000.0, 97_500.0, 97_000.0]);
        assert_eq!(
            channel(&log, "pressure").unwrap().quantity,
            Quantity::Pressure
        );
        assert!((values("temperature")[0] - (21.5 + 273.15)).abs() < 1e-9);
        assert_eq!(values("acceleration"), [-0.5, 100.0, -10.0]);
        assert_eq!(values("speed"), [0.0, 20.0, 40.0]);
        assert_eq!(log.altitude_m, [0.0, 4.0, 12.0]);
        assert_eq!(log.time_s, [1.0, 1.1, 1.2]);
        assert_eq!(values("accel"), [100.0; 3]);
        assert_eq!(values("sense_m"), [3.0; 3]);
        assert_eq!(
            channel(&log, "v_batt").unwrap().quantity,
            Quantity::Unrecognized
        );
        let events: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(events, ["pad", "boost", "state 9"]);
        assert_eq!(log.events[1].time_s, 1.1);
        assert_eq!(values("rssi"), [-80.0; 3]);
        assert_eq!(values("lqi"), [40.0; 3]);
    }

    /// Read as the document's tenths of a pascal, a pad at 98,000 would be 9.8 kPa: the files
    /// hold pascals. A pad outside 10–120 kPa read as pascals is kept as written.
    #[test]
    fn pressure_is_pascals_only_where_the_pad_bears_it_out() {
        let at = |pres: i32| read(&file(&[sensor(100, 2, pres, 0, 0, 0, 0)])).unwrap();
        for (pres, pascals) in [
            (10_000, true),
            (120_000, true),
            (9_999, false),
            (120_001, false),
            (980_000, false),
        ] {
            let log = at(pres);
            assert_eq!(channel(&log, "pressure").is_some(), pascals, "{pres}");
            assert_eq!(channel(&log, "pres").is_some(), !pascals, "{pres}");
        }
        assert_eq!(
            channel(&at(980_000), "pres").unwrap().quantity,
            Quantity::Unrecognized
        );
        // The pad state's median decides, not the flight's lowest pressure.
        let text = file(&[
            sensor(100, 2, 98_000, 0, 0, 0, 0),
            sensor(110, 3, 9_000, 0, 0, 0, 0),
            sensor(120, 4, 8_000, 0, 0, 0, 0),
        ]);
        assert!(channel(&read(&text).unwrap(), "pressure").is_some());
    }

    /// The tick turns over at 65,536: each tick on the turn nearest the latest so far, so a
    /// GPS packet's earlier tick is placed behind, and a turn adds 655.36 s.
    #[test]
    fn the_ticks_turns_are_unwound() {
        let mut gps = vec![0x10 | 7];
        gps.extend(1000i16.to_le_bytes());
        gps.extend(450_000_000i32.to_le_bytes());
        gps.extend((-1_220_000_000i32).to_le_bytes());
        let text = file(&[
            sensor(65_500, 2, 98_000, 0, 0, 0, 0),
            sensor(30, 2, 98_000, 0, 0, 0, 0),
            // Behind the latest by 40 ticks, and across the turn.
            line(65_526, LOCATION, &gps),
            sensor(32_000, 2, 98_000, 0, 0, 0, 0),
            sensor(64_000, 2, 98_000, 0, 0, 0, 0),
            sensor(100, 3, 98_000, 0, 0, 0, 0),
        ]);
        let log = read(&text).unwrap();
        let turn = 655.36;
        let expected = [
            655.0,
            turn + 0.30,
            turn + 320.0,
            turn + 640.0,
            2.0 * turn + 1.0,
        ];
        assert_eq!(log.time_s.len(), expected.len());
        for (time, expected) in log.time_s.iter().zip(expected) {
            assert!((time - expected).abs() < 1e-9, "{time} {expected}");
        }
        let latitude = channel(&log, "latitude").unwrap();
        assert!((latitude.time_s[0] - 655.26).abs() < 1e-9);
        assert!((latitude.values[0] - 45.0).abs() < 1e-12);
        assert!((channel(&log, "longitude").unwrap().values[0] + 122.0).abs() < 1e-12);
        assert_eq!(channel(&log, "gps_altitude").unwrap().values[0], 1000.0);
        assert_eq!(channel(&log, "nsat").unwrap().values[0], 7.0);

        // A step of 32,750 ticks after a GPS packet 40 behind: forward from the latest tick, so
        // within half a turn, though 32,790 from the GPS packet's would read as a step back.
        let text = file(&[
            sensor(1_000, 2, 98_000, 0, 0, 0, 0),
            line(960, LOCATION, &gps),
            sensor(33_750, 2, 98_000, 0, 0, 0, 0),
        ]);
        assert_eq!(read(&text).unwrap().time_s, [10.0, 337.5]);
    }

    /// A GPS packet without the valid bit has its position read as gaps, counted.
    #[test]
    fn a_position_without_a_fix_is_a_gap() {
        let mut gps = vec![0x20 | 3];
        gps.extend(1000i16.to_le_bytes());
        gps.extend(450_000_000i32.to_le_bytes());
        let log = read(&file(&[line(100, LOCATION, &gps)])).unwrap();
        assert!(channel(&log, "latitude").unwrap().values[0].is_nan());
        assert!(channel(&log, "gps_altitude").unwrap().values[0].is_nan());
        assert_eq!(channel(&log, "gps_flags").unwrap().values[0], 35.0);
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("1 GPS packets with the flags' valid bit"))
        );
        assert!(log.time_s.is_empty() && log.temperature_k.is_none());
        assert_eq!(log.logger, "AltOS");
    }

    /// A board switched off and on restarts its tick: the record keeps the power-on that leaves
    /// the pad, whether first or last, and refuses a file in which two do.
    #[test]
    fn the_power_on_that_flies_is_kept() {
        let earlier = [
            sensor(5_000, 2, 98_000, 0, 0, 0, 0),
            sensor(5_100, 2, 98_000, 0, 0, 0, 0),
        ];
        let flight = [
            sensor(500, 2, 98_000, 0, 0, 0, 0),
            sensor(510, 3, 97_000, 0, 0, 0, 10),
        ];
        let log = read(&file(&[&earlier[..], &flight[..]].concat())).unwrap();
        assert_eq!(log.time_s, [5.0, 5.1]);
        assert!(log.notes.iter().any(|n| n.contains("power-on 2 of 2 (the one that leaves the pad), and 2 packets")));
        // The flight first, then a power-on on the pad after landing, its tick behind.
        let after = [sensor(200, 2, 98_000, 0, 0, 0, 0)];
        let log = read(&file(&[&flight[..], &after[..]].concat())).unwrap();
        assert_eq!(log.time_s, [5.0, 5.1]);
        assert!(log.notes.iter().any(|n| n.contains("power-on 1 of 2")));
        // A power-on whose tick lands ahead of the last is a gap, not told apart.
        let log = read(&file(&[&flight[..], &earlier[..1]].concat())).unwrap();
        assert_eq!(log.time_s, [5.0, 5.1, 50.0]);
        // Neither leaves the pad: the last.
        let log = read(&file(&[&earlier[..], &flight[..1]].concat())).unwrap();
        assert_eq!(log.time_s, [5.0]);
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("the last, as none leaves the pad"))
        );
        // Both leave it: refused.
        let flight_again = [sensor(400, 3, 97_000, 0, 0, 0, 10)];
        let text = file(&[&flight[..], &flight_again[..]].concat());
        assert!(refusal(&text).contains("left the pad in two power-ons"));
    }

    /// The configuration gives the flight number and firmware; a type the document lays out but
    /// no sample holds, and one it doesn't define, are counted and not read.
    #[test]
    fn the_configuration_and_unread_types() {
        let mut config = vec![0x0b];
        config.extend(3u16.to_le_bytes());
        config.extend([1, 27]);
        config.extend(0u16.to_le_bytes());
        config.extend(250u16.to_le_bytes());
        config.extend(2048u16.to_le_bytes());
        config.extend(b"N0CALL\0\0");
        config.extend(b"1.9.22\0\0");
        let text = file(&[
            line(100, CONFIGURATION, &config),
            line(101, 0x08, &[0; 27]),
            line(102, 0x08, &[0; 27]),
            line(103, 0x30, &[0; 27]),
            sensor(104, 2, 98_000, 0, 0, 0, 0),
        ]);
        let log = read(&text).unwrap();
        assert_eq!(log.flight_number, Some(3));
        assert_eq!(log.firmware.as_deref(), Some("1.9.22"));
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("main at 250 m") && n.contains("\"N0CALL\""))
        );
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("2 packets of type 0x08 (TeleMega sensors) not read"))
        );
        assert!(log.notes.iter().any(|n| {
            n.starts_with("1 packets of type 0x30 not read: a type the document doesn't lay out")
        }));
        // A file of only unread types holds nothing to read.
        let text = file(&[line(101, 0x08, &[0; 27])]);
        assert!(matches!(read(&text), Err(LogError::NoData { .. })));
    }

    /// The document's own example line (*AltOS Telemetry*, §4, p. 10), not built by this file's
    /// helpers: its checksum passes as the reader sums it, and read little-endian it is a GPS
    /// fix from serial 335 at 45.4696816° N, 122.7376450° W, six satellites, the valid bit set,
    /// its RSSI byte 0x3F, 63, so −42.5 dBm, the CRC bit set.
    #[test]
    fn the_documents_example_line_reads() {
        const EXAMPLE: &str =
            "TELEM 224f01080b05765e00701f1a1bbeb8d7b60b070605140c000600000000000000003fa988";
        let log = read(EXAMPLE).unwrap();
        assert_eq!(log.serial_number.as_deref(), Some("335"));
        let value = |column: &str| channel(&log, column).unwrap().values[0];
        assert!((value("latitude") - 45.469_681_6).abs() < 1e-12);
        assert!((value("longitude") + 122.737_645_0).abs() < 1e-12);
        assert_eq!(value("gps_altitude"), 94.0);
        assert_eq!(value("nsat"), 6.0);
        assert_eq!(value("gps_flags"), f64::from(0x76u8));
        assert_eq!(value("rssi"), -42.5);
        assert_eq!(value("lqi"), f64::from(0xa9u8 & 0x7f));
        // The clock: tick 0x0b08, 2,824.
        assert_eq!(channel(&log, "latitude").unwrap().time_s[0], 28.24);
        // Summed from the length byte, as the document's formula reads, the line fails.
        let hex = EXAMPLE.strip_prefix("TELEM ").unwrap();
        let bytes: Vec<u8> = (0..hex.len() / 2)
            .map(|k| u8::from_str_radix(&hex[2 * k..2 * k + 2], 16).unwrap())
            .collect();
        let from_length = bytes[..35]
            .iter()
            .fold(CHECKSUM_BASE, |sum, b| sum.wrapping_add(*b));
        assert_ne!(from_length, bytes[35]);
    }

    /// A power-on heard only in the `landed` state, as after a silence once the rocket is down,
    /// isn't a second flight: the one that flew is kept.
    #[test]
    fn a_power_on_heard_only_landed_is_no_second_flight() {
        let text = file(&[
            sensor(500, 2, 98_000, 0, 0, 0, 0),
            sensor(510, 3, 97_000, 0, 0, 0, 10),
            sensor(520, 7, 97_500, 0, 0, 0, 5),
            // A step back of 25,536 ticks after a silence of 400 s: read as a power-on.
            sensor(40_600, 8, 98_000, 0, 0, 0, 0),
        ]);
        let log = read(&text).unwrap();
        assert_eq!(log.time_s, [5.0, 5.1, 5.2]);
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("power-on 1 of 2 (the one that leaves the pad)"))
        );
    }

    /// Configuration packets naming two firmware versions keep the first, with a note.
    #[test]
    fn differing_firmware_keeps_the_first() {
        let config = |version: &[u8; 8]| {
            let mut data = vec![0x0b];
            data.extend(3u16.to_le_bytes());
            data.extend([1, 27, 0, 0, 250, 0, 0, 8]);
            data.extend(b"N0CALL\0\0");
            data.extend(version);
            data
        };
        let text = file(&[
            line(100, CONFIGURATION, &config(b"1.9.22\0\0")),
            line(200, CONFIGURATION, &config(b"1.9.23\0\0")),
        ]);
        let log = read(&text).unwrap();
        assert_eq!(log.firmware.as_deref(), Some("1.9.22"));
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("the configuration packets name more than one firmware"))
        );
    }

    /// The step between two ticks is on the nearest turn.
    #[test]
    fn a_step_is_on_the_nearest_turn() {
        assert_eq!(step(65_500, 30), 66);
        assert_eq!(step(30, 65_500), -66);
        assert_eq!(step(0, 32_767), 32_767);
        assert_eq!(step(0, 32_768), -32_768);
    }
}
