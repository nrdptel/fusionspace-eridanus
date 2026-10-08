# Smartwatches: a scoping note for later

**Bottom line:** a watch app is a small companion to the phone, not a place to run the simulator.
Every use worth having on a wrist is a glance or an alert: the walk to the rocket, the flight's
events, a countdown. The phone does the computing and relays a few small messages. Nothing is
built yet. It will be researched when the time comes, which is after
the mobile apps ([M9.4, mobile][m9-4]). This note records what today's choices must keep possible
([ADR-191, the 2026-10-06 ideas][adr-191]). Researched October 6, 2026.

## Platforms

| platform | language | can it run hpr's Rust? | limits that matter |
|---|---|---|---|
| Apple Watch (watchOS 26) | Swift, SwiftUI | yes: `aarch64-apple-watchos` is a Tier 2 Rust target; `arm64_32-apple-watchos`, still needed for Series 6 to 8, SE 2 and Ultra 1, is Tier 3 (nightly only) ([rustc](https://doc.rust-lang.org/stable/rustc/platform-support.html)) | apps suspend quickly; a workout session keeps Bluetooth alive ([WWDC22](https://developer.apple.com/videos/play/wwdc2022/10135/)) |
| Wear OS | Kotlin, Compose | yes, through the NDK; many watches still run 32-bit ARM apps ([Android](https://android-developers.googleblog.com/2026/04/get-your-wear-os-apps-ready-for-64-bit-requirement.html)) | long Bluetooth links need a foreground service, which the system can still stop ([Android](https://developer.android.com/training/wearables/versions/5/changes)) |
| Garmin Connect IQ | Monkey C | no: no native code | tens of KB to a few MB of memory; Bluetooth central from API 3.1 ([Garmin](https://developer.garmin.com/connect-iq/api-docs/Toybox/BluetoothLowEnergy.html)) |
| Pebble (Core 2 Duo, Time 2) | C | a `no_std` piece could | 256 KB RAM; niche ([CNX](https://www.cnx-software.com/2025/03/19/the-pebble-smartwatch-is-back-with-the-core-2-duo-and-core-time-2-models-running-pebbleos-open-source-firmware/)) |

## Uses, most valuable first

1. **The walk to the rocket:** bearing and distance from the watch's GPS and compass to the
   tracker's last fix. The geodesy can run on the watch; the fix comes through the phone.
2. **Haptic alerts** for liftoff, apogee, each deployment, landing and a lost signal.
3. **A live glance** at the tracker: altitude, speed, link quality, satellites.
4. **The pad countdown and checklist,** with a tap at each step; synced from the phone.
5. **The flight card at a glance:** motor, delay, predicted apogee and drift, as estimates with
   their ranges.
6. **A wind reading entered at the pad,** sent to the phone, which re-flies the drift.

Only a re-flight with no phone nearby would need the simulator on the watch; that isn't worth the
build cost. A ground station that accepts only one Bluetooth connection, held by the phone, would
also route the watch through the phone; whether the common receivers do is unchecked.

Featherweight's phone app lists the Apple Watch as compatible
([App Store](https://apps.apple.com/us/app/featherweight-ui/id1641310058)); DJI's drone app added
a watch companion in December 2025
([DroneDJ](https://dronedj.com/2025/12/11/dji-drone-apple-watch-control/)). No other rocketry
watch app was found.

## What today's work must keep possible

- **The phone is the hub.** The UI architecture decision ([M9.0][m9-0]) names which part of the
  mobile app owns the watch link. A PWA can't host a watch extension, so the link needs the native
  shell.
- **A compact event and position message** in the planned telemetry crate
  ([M13.1, ground station][m13-1]): fixed layout, versioned, a few dozen bytes, simple enough to
  decode by hand on Connect IQ, and mapping to the canonical flight record
  ([M7.1, flight log importers][m7-1]).
- **Small shared pieces in `no_std` crates:** bearing and distance on WGS84 and the event detector,
  shared by firmware, phone and watch.
- **No 64-bit assumptions** in those crates; the existing `wasm32` build check already catches most.
- **The flight card as a small summary file,** so the watch never reads a design.

[adr-191]: ../decisions/0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[m7-1]: ../decisions-and-roadmap.md#m7-1
[m9-0]: ../decisions-and-roadmap.md#m9-0
[m9-4]: ../decisions-and-roadmap.md#m9-4
[m13-1]: ../decisions-and-roadmap.md#m13-1
