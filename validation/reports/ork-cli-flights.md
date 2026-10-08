# The `.ork` survey through `hpr sim`

Written by `cargo xtask ork-cli` (M4.5j); don't edit it by hand. Every motor configuration of every `.ork` in `cargo xtask ork`'s survey (the design library under `refs/` and the examples inside OpenRocket 24.12's jar) is flown by `hpr sim --offline`, once the motors it fetches from ThrustCurve.org are cached. A configuration refused as saved is flown again with `--accept-design-errors`. The library is other people's designs, so it is counted, not listed; a file found in two places is counted in each.

`hpr sim` flies 136 of the survey's 170 motor configurations as saved, offline after one fetch, and 9 more with `--accept-design-errors`; 25 are refused. Of OpenRocket's 56 example configurations, 54 fly as saved, 0 more with `--accept-design-errors`, and 2 are refused.

## By source

| Source | Files | Unreadable | No configuration | Configurations | Fly | Fly with the flag | Refused |
|---|---:|---:|---:|---:|---:|---:|---:|
| OpenRocket's examples | 17 | 0 | 0 | 56 | 54 | 0 | 2 |
| elsewhere under refs/ | 31 | 2 | 10 | 25 | 5 | 0 | 20 |
| refs/loft-fixtures | 27 | 0 | 0 | 89 | 77 | 9 | 3 |
| **All** | 75 | 2 | 10 | 170 | 136 | 9 | 25 |

## Refused, by reason

| Reason | Configurations |
|---|---:|
| a hybrid motor (out of scope until release 1.0) | 3 |
| a motor with no curve fetched: ThrustCurve.org had none by its name, or the fetch failed | 1 |
| a motor with no curve whose file records no digest, so none is fetched | 20 |
| a separation hpr doesn't fly | 1 |

## OpenRocket's examples

| File | Configuration | Motors | Outcome |
|---|---:|---|---|
| 3D printable nose cone and fins.ork | 1 | A8 | flies |
| 3D printable nose cone and fins.ork | 2 | B6 | flies |
| 3D printable nose cone and fins.ork | 3 | C6 | flies |
| 3D printable nose cone and fins.ork | 4 | C6 | flies |
| 3D printable nose cone and fins.ork | 5 | C6 | flies |
| A simple model rocket.ork | 1 | A8 | flies |
| A simple model rocket.ork | 2 | B4 | flies |
| A simple model rocket.ork | 3 | C6 | flies |
| A simple model rocket.ork | 4 | C6 | flies |
| A simple model rocket.ork | 5 | C6 | flies |
| ARC payload rocket.ork | 1 | F50T | flies |
| Airstart timing.ork | 1 | K550W; I211W | flies |
| Airstart timing.ork | 2 | K550W; I211W | flies |
| Airstart timing.ork | 3 | K550W; I211W | flies |
| Airstart timing.ork | 4 | K550W; I211W | flies |
| Airstart timing.ork | 5 | K550W; I211W | flies |
| Base drag hack (short-wide).ork | 1 | C11 | flies |
| Base drag hack (short-wide).ork | 2 | D12 | flies |
| Base drag hack (short-wide).ork | 3 | E12 | flies |
| Chute release.ork | 1 | G40W | flies |
| Chute release.ork | 2 | G80T | flies |
| Clustered motors.ork | 1 | A8 | flies |
| Clustered motors.ork | 2 | B4 | flies |
| Clustered motors.ork | 3 | C6 | flies |
| Clustered motors.ork | 4 | C6 | flies |
| Clustered motors.ork | 5 | C6 | flies |
| Deployable payload.ork | 1 | A8 | flies |
| Deployable payload.ork | 2 | B4 | flies |
| Deployable payload.ork | 3 | C6 | flies |
| Deployable payload.ork | 4 | C6 | flies |
| Deployable payload.ork | 5 | C6 | flies |
| Dual parachute deployment.ork | 1 | H669N | flies |
| Dual parachute deployment.ork | 2 | H242T | flies |
| Dual parachute deployment.ork | 3 | J570W | flies |
| Dual parachute deployment.ork | 4 | H999N | flies |
| Dual parachute deployment.ork | 5 | I1299N | flies |
| Dual parachute deployment.ork | 6 | G64W | flies |
| Parallel booster staging.ork | 1 | E12; I115W | flies |
| Parallel booster staging.ork | 2 | I115W | flies |
| Pods--airframes and winglets.ork | 1 | A8 | flies |
| Pods--airframes and winglets.ork | 2 | B6 | flies |
| Pods--airframes and winglets.ork | 3 | C6 | flies |
| Pods--airframes and winglets.ork | 4 | C12 | flies |
| Pods--airframes and winglets.ork | 5 | D16 | flies |
| Pods--powered with recovery deployment.ork | 1 | C6; B6; A3; A3 | flies |
| Pods--powered with recovery deployment.ork | 2 | C6; B6 | flies |
| Pods--powered with recovery deployment.ork | 3 | C6 | flies |
| Pods--powered with recovery deployment.ork | 4 | B6; A10; A10 | flies |
| Simulation extensions.ork | 1 | 2800CC172L-L540 | refused: a hybrid motor (out of scope until release 1.0) |
| Simulation scripting.ork | 1 | 2800CC172L-L540 | refused: a hybrid motor (out of scope until release 1.0) |
| Three stage low power rocket.ork | 1 | A8; B6; B6 | flies |
| Three stage low power rocket.ork | 2 | C6; B6; B6 | flies |
| Three stage low power rocket.ork | 3 | C6; C6; C6 | flies |
| Tube fin rocket.ork | 1 | D12 | flies |
| Two stage high power rocket.ork | 1 | H148R; H148R | flies |
| Two stage high power rocket.ork | 2 | I59WN; I357T | flies |
