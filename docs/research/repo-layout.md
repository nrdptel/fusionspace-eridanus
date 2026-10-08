# One repository, several workspaces: when and how hpr splits

**Bottom line:** hpr stays one repository. Its parts that need another toolchain (firmware, the
desktop and mobile shells, the web app) each become their own Cargo workspace or build directory
inside it. Only two things get a repository of their own: a service that runs on a server, and a
part whose license isn't permissive. No git submodules. This note backs
[ADR-191, the 2026-10-06 ideas][adr-191], which amends the "one workspace" line of
[ADR-162, the suite and its releases][adr-162]. Researched October 6, 2026; the measured figures
are from that day's `main`.

The same day, Neer allowed splitting into smaller workspaces as the products grow, be that
separate directories, separate repositories or sub-repositories, and asked for it to be
researched.

## Where the repository stands

| measure | October 6, 2026 |
|---|---|
| crates in the one workspace | 18, plus `xtask` (3 are stubs) |
| Rust lines tracked | 248,000 (204,000 in `crates/`, 44,000 in `xtask`) |
| packages in `Cargo.lock` | 282 |
| tracked files; `.git` size | 1,139; 92 MB |
| CI per push to `main` (last 10 green runs) | 20 jobs, about 77 runner-minutes, 16.7 min mean wall time (13.1 to 26.2) |
| slowest job | the Windows test job, 13.9 min mean |

CI has no path filters today. The repository is public, so runner minutes cost nothing.

## The options

| option | a change across parts | CI cost | fits hpr? |
|---|---|---|---|
| one workspace | atomic | every job builds everything | today; breaks when firmware joins, since `thumbv7em` needs its own target and profile |
| several workspaces, one repository | atomic, through path dependencies | each has its own lock file and `target/` | yes: the plan |
| a repository per part | not atomic; versions drift | smallest | only for a service or a different license |
| git submodules | not atomic; detached checkouts, forgotten pushes ([Pro Git](https://git-scm.com/book/en/v2/Git-Tools-Submodules)) | low | no: scripted work in one checkout trips on stale pins |
| subtree or josh mirrors | synced both ways ([Rust's josh notes](https://rustc-dev-guide.rust-lang.org/external-repos.html)) | low | later, for a read-only mirror of one part |

Cargo can't nest workspaces ([cargo#5042](https://github.com/rust-lang/cargo/issues/5042)), so each
extra workspace is a top-level directory. A required CI check from a workflow skipped by `paths:`
stays pending and blocks a merge
([GitHub](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/troubleshooting-required-status-checks)).
So filters go in a `changes` job plus one always-run summary check, never in `paths:`.

## What similar projects did

- **One repository:** Bevy (one workspace, CI written in Rust), Ruffle (core, desktop and web
  together), OpenRocket (a GUI-free core module and a Swing module), Zed (about 370 crates; its
  users report long links and heavy memory use).
- **Split by codebase and team:** PX4 and QGroundControl, ArduPilot and Mission Planner,
  Betaflight and its configurator. PX4's submodules are a known nuisance
  ([PX4 forum](https://discuss.px4.io/t/guide-dealing-with-annoying-submodule-changes-when-switching-branch/29059)).
- **Split for packaging:** Tauri 2 moved its plugins to their own workspace and keeps read-only
  mirrors, because JavaScript package managers can't install from a monorepo.
- **Firmware as separate Cargo projects:** Embassy has no root workspace; each board's example is
  its own project with a path dependency ([Embassy](https://github.com/embassy-rs/embassy)).
- **Many repositories:** RocketPy (library, API, serializer and website apart).

hpr differs from the firmware projects in one way: its flight computer runs the same `no_std`
crate that the simulator runs in the loop (`hpr-embedded`). A repository boundary there would
break the guarantee that the two agree, so the boundary is a workspace.

## The target layout

```text
fusionspace-eridanus/
  Cargo.toml     the root workspace: crates/* and xtask (core, CLI, Python, WASM,
                 analyzer; later hpr-propulsion, hpr-chute, hpr-embedded, hpr-telemetry)
  apps/web/      the PWA on hpr-wasm, with the rebuilt Charge, Window, Muster and stock screens
  apps/desktop/  the Tauri shell, its own workspace
  apps/mobile/   iOS and Android, with any watch targets inside
  firmware/      its own workspace, a path dependency on crates/hpr-embedded
  hardware/      KiCad sources and printable CAD, its license marked per directory
  docs/ validation/ schema/ scripts/
```

Motor Finder's scraper stays in its own repository: it runs on a server every hour and needs
secrets ([the web tools note](suite-web-tools.md)).

## Split triggers

A part moves only when a trigger fires. The trigger is recorded in the ADR that moves it.

| # | trigger, measured | what moves |
|---|---|---|
| T1 | mean wall time of a core-only PR above 25 min (16.7 today), or a non-core job adds more than 3 min to one | add the `changes` job and gate non-core jobs; nothing moves |
| T2 | a part needs another target, profile, toolchain or build system (`thumbv7em`, Xcode, Gradle, pnpm) | its own top-level workspace or directory, its own workflow; its filter includes the crates it uses |
| T3 | one member adds more than half again to `Cargo.lock`, or needs `deny.toml` exceptions only it uses | out of the root workspace (likely the desktop shell) |
| T4 | a license other than MIT, Apache-2.0, CERN-OHL-P or CC BY | its own repository, so the root's license checks stay simple |
| T5 | runs on a server: deploys continuously, needs secrets or a schedule | its own repository, read through `hpr-net`'s cache with an offline fallback |
| T6 | versioned apart from the suite (board revisions, app-store builds) more than about twice a quarter | per-package releases, same repository |
| T7 | steady outside contributors to one part, or a package manager that can't install from here | a read-only mirror, not a move |
| T8 | the local `target/` above 50 GB, or the gate above 30 min on the Mac | a smaller `default-members` set, before splitting anything |

T2 fires first: when the firmware's first crate lands ([M13.2, GPS tracker logic][m13-2]), and when
the app's first shell lands ([M9.0, the UI architecture][m9-0]).

[adr-162]: ../decisions/0162-the-suite-and-its-releases.md
[adr-191]: ../decisions/0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[m9-0]: ../decisions-and-roadmap.md#m9-0
[m13-2]: ../decisions-and-roadmap.md#m13-2
