# Can OpenRocket's edge snap serve as M2.3c3's pinned oracle

**In short: yes as a pinned, runnable binary; no as a release.** Every OpenRocket build tried, back to revision 2
of 2020, downloads today by a plain `curl -L`, with no login and no snapd, from an address that names the revision;
the five downloaded match the sha3-384 that Canonical signs for each [S1, S2, S3]. 7-Zip takes the jar out in about
a second [S9]. The jar flies headless on macOS on Java 17 and 21,
but only the *universal* jar of the armhf, ppc64el or s390x snaps: the amd64 and arm64 snaps carry Linux-only jars
that load no motors on a Mac [S6, S9]. The scripts' 24.12 start-up fails on it (a motor-database update dialog);
a start without the graphical module works [S9]. On OpenRocket's own two examples the edge build's apogee is
0.25% and 1.08% below 24.12's [S9]. By OpenRocket's own naming the build is a development snapshot
(`26.xx-SNAPSHOT-<commit>`), not a release or pre-release [S6, S7], and the store promises no retention [S4, S5].
So it does not meet [ADR-212](../decisions/0212-measure-against-openrockets-newest-release.md)'s "release
published after 24.12" or [M2.3c3, OpenRocket's newest release](../decisions-and-roadmap.md#m2-3c3); flying it
for either needs a new ADR (§6).

**What this covers.** The snap store's public API, its download endpoint and signed records, Canonical's forum and
terms, OpenRocket's build guide, releases and downloads pages, and the edge jar run as an external oracle on a Mac.
Only file names, the jar's manifest, `build.properties`, the motor database's metadata and the jar's public class
names were read; no OpenRocket source, diff or class body. It builds on
[What OpenRocket changed after 24.12](openrocket-after-24-12.md) §1 and §8. **How far to trust it.** Store
behavior and file contents were measured on 2026-10-20; the store's anonymous revision access is observed, not
documented, and could change. The Linux jar was not run on Linux. Two example designs size nothing for the
comparison sets. Every **Inference** is marked.

## 1. What is on the store today

The info endpoint (`api.snapcraft.io/v2/snaps/info/openrocket`, header `Snap-Device-Series: 16`) lists 16
channel and architecture entries for snap id `6Io6dRgbbYTm1cPXEQYbLz1iLF9zh2k7`, publisher `openrocket`, license
GPL-3.0, one track, no candidate channel [S1]:

| Channel | Builds | Version | Date (UTC) |
|---|---|---|---|
| stable | revisions 1361 to 1365, five architectures | 24.12 | 2025-11-09 |
| beta | revisions 1330 to 1334 | 24.12 | 2025-10-23 |
| edge | amd64 1770, armhf 1773 | `26.xx-g44feea3` | 2026-10-07 |
| edge | arm64 1775, ppc64el 1776, s390x 1774 | `26.xx-g591c5e5` | 2026-10-08 |
| edge | i386 1238 | 22.02 (stale) | 2023-06-27 |

- `g44feea3` is the merge of OpenRocket's #3382 (2026-10-07 10:18 UTC), `g591c5e5` that of #3296 (14:31 UTC)
  [S8]; amd64 and armhf stay on the older one because their `g591c5e5` builds failed on Launchpad [S10].
- 24.12 reached the stable channel on 2025-11-09, 105 days after its GitHub release of 2025-07-27 [S1, S8].

## 2. Pinning: a stable address and a hash

- **Address.** `https://api.snapcraft.io/api/v1/snaps/download/6Io6dRgbbYTm1cPXEQYbLz1iLF9zh2k7_<rev>.snap`
  answers a bare `curl` with a 302 to a CDN address, then 200 with the file; range requests work [S2]. The CDN
  address's token carries an expiry time about 3 hours ahead, so only the api.snapcraft.io address can be pinned.
  It takes the snap id; the name (`openrocket_1770.snap`) gives 404 [S2].
- **Off-channel revisions.** About 20 revisions on no channel today (1769, 1700, 1500, 1000, 300, 10, 2 and others)
  answer the same way [S2]. The refresh API, asked anonymously for revisions 1 to 1780, served 1,448 and reported
  332 not found (1777 to 1780 are past the newest); each served one names the channel it was released to (1,433
  edge, 11 stable, 4 beta) [S3]. Whether a never-released revision is served could not be told.
- **Documentation and history disagree.** In 2022 the `--help` text, as quoted on the forum, gave `--revision` to
  snaps "to which you must have developer access", and anonymous requests by revision were refused [S11]. A forum
  reply of June 2024 reports the change, for stable revisions, and suggests asking whether the store team could
  restore the old rule [S12]. Today the three revisions refused in 2022, and edge and beta revisions of other snaps,
  download anonymously [S3]. So anonymous revision access is a behavior since about 2023 to 2024, not a promise.
- **Hash.** The store publishes sha3-384, not SHA-256. Downloads of 1770 and 1775 reproduce it exactly; SHA-256
  of 1770 is `c9e735ab…04a0a5c` and of 1775 `41d11d0c…2d43cbfd` [S2]. Canonical's signed `snap-revision` record
  (revision, size, sha3-384, publisher) downloads anonymously from `assertions.ubuntu.com`, with no SHA-256 or
  commit [S3]. Python's `hashlib.sha3_384` is guaranteed on every platform but a FIPS-restricted build [S13];
  macOS's built-in `openssl` (LibreSSL 3.3.6) and PowerShell's `Get-FileHash` have no SHA3 [S2, S14].

## 3. Retention

- **No policy found.** The Snap Store terms (October 2020) let Canonical remove a snap "at any time in its sole
  discretion" and let a publisher remove one; they state no retention period [S4]. A Canonical address wrote in
  2017 that the store does not "generally remove revisions"; a 2019 store reply said there is "no expiration"
  today but reaping unpublished revisions was "being considered and designed" [S5]. Canonical's revisions page
  says only that the store "caches several older revisions of every snap" [S16].
- **Observed.** Revision 2 (`20-08-alpha-15`, 2020-10-15) still downloads, six years on [S3]. The missing numbers
  cluster in 301 to 603 (2022) and 1307 to 1324 (2024 to 2025); removed or never uploaded can't be told.
- **Cadence.** Edge builds come from a Launchpad recipe that uploads to edge without a hand step [S6, S10]; 93
  distinct commits reached edge from 2025-11-09 to 2026-10-08, a median of 2.0 days apart, the longest gap 36 days
  [S3]. Of the 300 newest builds (60 build requests), 11 failed [S10]. Nothing has merged since 2026-10-08, so
  edge has not moved for 12 days [S8].
- **Inference:** a pinned revision should last years unless the store starts reaping, the snap is removed, or
  anonymous revision access ends; none has a date. A copy of the `.snap` cached under `refs/` covers all three,
  and GPL-3.0 allows keeping the binary, as the 24.12 jar is kept.

## 4. Unpacking without snapd

- Every edge snap is a squashfs 4.0 image, xz-compressed, 131,072-byte blocks, read from its header [S9].
- **7-Zip 26.04** (`7zz`; Linux, macOS, Windows; LGPL-2.1 and BSD-3-Clause) extracts the jar in 0.93 s.
  **PySquashfsImage 0.9.0** (pure Python, LGPL) gives the same bytes in 1.55 s. `unsquashfs` (squashfs-tools,
  GPL-2.0, Homebrew `squashfs` 4.7.5) was not installed here and not tried [S9]. Each runs as an outside program
  or library, never vendored; **Inference:** none touches the crates' license, and `cargo deny` sees no Python.
- The jar sits at the snap's root, `/OpenRocket.jar`; the snap unpacks to 14,719 files, 1.09 GB [S9].
- **Manifest:** `Implementation-Version: 26.xx-SNAPSHOT`, `Main-Class: info.openrocket.swing.startup.OpenRocket`;
  `build.properties` gives `26.xx-SNAPSHOT-44feea394` (1770, 1773) or `-591c5e5f7` (1774 to 1776). Package names
  are unchanged from 24.12 (`info.openrocket.core`, `info.openrocket.swing`) [S9].
- **Bundled Java:** Ubuntu OpenJDK 17.0.20.1 and 11.0.32.1, Linux builds, with `JAVA_HOME` set to the 17 [S9].

## 5. Which jar runs, and how

**Platform jars.** The build guide: snaps "select the Linux variant on amd64 and arm64, while other Snap
architectures keep using the universal JAR" [S6]. Measured: all 35,872 entries of the amd64 jar match the armhf
jar's by CRC and size; the armhf jar adds 172 entries (76 native libraries and their folders) for macOS, Windows,
FreeBSD and other CPUs [S9]. So amd64 1770's jar (89,226,845 bytes, SHA-256 `d2f46918…`) is Linux-only, while
armhf 1773's (105,980,333, `5cb5d328…`) and ppc64el 1776's (105,981,323, `61c37446…`) are universal [S9].
Same-commit jars hold the same content but not the same bytes (ppc64el and s390x differ by 16,281,758 bytes), so
a pin must name the snap revision, not just the commit [S9].

**Runs on an arm64 Mac** (JPype 1.7.1, `-Djava.awt.headless=true`, a scratch `user.home`) [S9]:

| Run | Result |
|---|---|
| 24.12, the scripts' graphical-module start, Java 17 or 21 | both examples fly, same numbers on 17 and 21 |
| edge (1770 or 1773), the same start | `HeadlessException`: start-up opens a motor-database update dialog |
| edge 1770 (Linux jar), core-only start | no SQLite library for macOS: 0 motors, nothing flies |
| edge 1773, 1774 or 1776, core-only start, Java 17 (1773 also 21) | 1,458 motors in under 1 s; both fly |

- **The core-only start** binds the motor database by hand through public classes (`MotorDatabaseInitializer`,
  `MotorDatabaseLoader`); 24.12 has no initializer, so the scripts need a branch by version [S9].
- **macOS only:** the preferences must be built on the main thread first, or the loader thread hangs in an AppKit
  call; a thread dump shows it [S9]. **Inference:** Linux and Windows have no such main-thread rule.
- **Network:** with a dead proxy the core-only start logged no connection attempt; the online update sits behind
  the graphical start [S9]. The loader prefers a `motors.db` in the user's OpenRocket folder, so a machine where the
  26.xx interface has updated its database would fly other motor data: each run needs a scratch `user.home`.
- **Shared settings:** both versions write one macOS Java preferences node that `user.home` does not move [S9].
  **Inference:** the scripts set every option they record, so no recorded number depends on it.
- **Java:** 24.12 and edge 1773 flew identically on Java 17.0.20.1 and 21.0.12.1, though `docs/VALIDATION.md`
  says 24.12 "refuses 21". **Inference:** that refusal lives in the graphical launcher.
- **Motor data changed format:** 24.12 ships a serialized motor file; the edge jar ships an SQLite database,
  version 20260418111610, 1,458 motors and 1,591 curves [S9].

**First numbers** (calm air, seed 1, each design's first stored simulation, designs from inside each jar) [S9]:

| OpenRocket example | 24.12 apogee | Edge apogee | Edge less 24.12 |
|---|---|---|---|
| A simple model rocket | 319.136 m | 318.350 m | −0.786 m (−0.25%) |
| Two stage high power rocket | 678.453 m | 671.146 m | −7.307 m (−1.08%) |

The three universal jars (two commits) give bit-identical numbers. **Inference:** the lower apogees fit the
fin–body interference the older note's §3 describes, but the motor data changed too, so they are not one cause.

## 6. Does an edge build meet ADR-212?

- **OpenRocket's own scheme.** Releases are numbered `YY.MM` and tagged `release-<version>` on GitHub; alpha, beta
  and candidate builds are marked pre-release there; development snapshots carry a version "ending in
  `-SNAPSHOT`" [S6]. The newest release is still 24.12 (2025-07-27), the newest pre-release 24.12.RC.01 [S8], and
  the downloads page offers v24.12 only [S7]. Snapcraft describes edge builds as "typically built automatically by
  a CI process from an arbitrary source code snapshot" [S15].
- **Answer.** The edge build is a development snapshot, neither release nor pre-release, and nothing publishes a
  SHA-256 of its jar. It meets neither ADR-212 §1 ("each OpenRocket release published after 24.12 … pinned by its
  published address and SHA-256") nor M2.3c3's "newest release after 24.12". Flying it for M2.3c3, or counting it
  in M10.3's bar, would change a done-when, so it needs a new ADR.

## 7. What this suggests

- **Keep M2.3c3 waiting, but make it a fetch away.** The start-up branch of §5 (core-only start, scratch
  `user.home`, main-thread preferences on macOS, the motor database's version recorded) is needed for any 26.xx
  jar, release or not. Expected effect: when 26.xx ships, M2.3c3 is a pin and a run, not a porting job.
- **If an ADR flies the snapshot** (a suggestion, not a plan), it would: name it "OpenRocket 26.xx development
  snapshot `<commit>`, snap revision N", never "26.xx"; fly it beside 24.12 on the flights both fly, outside
  M10.3's bar and any "better than OpenRocket" claim (ADR-209 §2); pin the revision address, the store's
  sha3-384, the jar's path and the project's measured SHA-256; use the universal jar (today armhf 1773 at
  `g44feea3`, or ppc64el 1776 at `g591c5e5`); name the extraction tool; replace the row when a release ships.
  Expected effect: a measured size for the version gap before M2.7 fixes its predictions.
- **The lock file** pins whole files by SHA-256; a snap pin needs an "extract this path from this archive" step
  and a sha3-384 check through Python. About 250 MB is fetched for a 106 MB jar.
- **Cache the `.snap`.** The store promises nothing (§3); the cached copy under `refs/` is the fallback.

## Open questions

- Whether the Linux amd64 jar flies on Linux amd64, CI's platform: not run (**Inference:** yes, it has SQLite).
- How far the edge and 24.12 apogees part on the public OpenRocket sets and the collection: not run, since the
  scripts need §5's start-up first.
- Where `docs/VALIDATION.md`'s "refuses 21" comes from; the graphical launcher was not run.
- Whether a never-released revision downloads, and whether the store will reap revisions or end anonymous access.

## Sources

All read or measured 2026-10-20; copies in the gitignored `refs/research/openrocket-edge-snap-oracle/`.

1. **S1.** Snap store info API. <https://api.snapcraft.io/v2/snaps/info/openrocket> (header `Snap-Device-Series: 16`)
2. **S2.** Snap download endpoint, by revision; HEAD, range and full downloads of 1770 and 1775 with hashes.
   <https://api.snapcraft.io/api/v1/snaps/download/6Io6dRgbbYTm1cPXEQYbLz1iLF9zh2k7_1770.snap>
3. **S3.** Snap store refresh API (`POST /v2/snaps/refresh`, download by revision 1 to 1780) and the assertions
   service. <https://api.snapcraft.io/docs/refresh/>; <https://assertions.ubuntu.com/v1/assertions/snap-revision/>
4. **S4.** Store terms (2020) §4.3, 4.4, 10. <https://canonical.com/legal/terms-and-policies/snap-store-terms>
5. **S5.** Snapcraft list, 2017-03-13 <https://lists.snapcraft.io/archives/snapcraft/2017-March/003519.html>;
   forum, 2019-10-17 <https://forum.snapcraft.io/t/optional-expiration-date-on-revisions/13751>
6. **S6.** OpenRocket, Building and Releasing (version numbers, platform jars, snap).
   <https://openrocket.readthedocs.io/en/latest/dev_guide/building_releasing.html>
7. **S7.** OpenRocket downloads. <https://openrocket.info/downloads.html>
8. **S8.** GitHub API: OpenRocket releases; pull requests #3382 and #3296 (metadata only); merged since 2026-10-08.
   <https://github.com/openrocket/openrocket/releases>
9. **S9.** Measured on an arm64 Mac: snaps 1770, 1773, 1774, 1775 and 1776 unpacked with `7zz` and PySquashfsImage;
   manifests, `build.properties`, motor metadata, public class names; JPype runs of 24.12 and the edge jars.
10. **S10.** Launchpad builds. <https://api.launchpad.net/devel/+snaps?ws.op=findByStoreName&store_name=openrocket>
11. **S11.** Snapcraft forum, 2022.
    <https://forum.snapcraft.io/t/access-by-specifying-a-revision-is-not-allowed-for-this-snap/29341>;
    <https://forum.snapcraft.io/t/cannot-download-my-own-snap-by-revision/31209>
12. **S12.** Snapcraft forum, reply of 2024-06-18. <https://forum.snapcraft.io/t/snapcraft-remove-older-revisions/40645>
13. **S13.** Python `hashlib`, `algorithms_guaranteed`. <https://docs.python.org/3/library/hashlib.html>
14. **S14.** `Get-FileHash`.
    <https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/get-filehash>
15. **S15.** Snapcraft, Channels and tracks. <https://snapcraft.io/docs/explanation/how-snaps-work/channels-and-tracks/>
16. **S16.** Snapcraft, Revisions. <https://snapcraft.io/docs/explanation/how-snaps-work/revisions/>
