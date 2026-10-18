//! The public designs the synthetic flights fly, each with its RocketPy example's own parachutes.
//!
//! The ten are every design under `validation/designs/` that is a published example and can fly:
//! RocketPy's examples, rebuilt as HPR designs from `validation/oracles/rocketpy/rocket_mass.py`
//! (`validation/designs/README.md`). The others there have no motor (the MIL-HDBK-762 sample, the
//! two wind-tunnel models) or are invented (the two synthetic rockets). The designs leave their
//! parachutes out, since RocketPy gives only their drag area; the drag areas, triggers and lags
//! below are each example's own, as `rocket_mass.py` records them, at the lines given.

use std::f64::consts::PI;

use serde::Serialize;

use super::SyntheticError;

/// The inch, m.
const INCH_M: f64 = 0.0254;
/// The foot, m.
const FOOT_M: f64 = 0.3048;

/// What opens a parachute.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Opens {
    /// At apogee.
    Apogee,
    /// Descending through this height above the launch site, m, or at apogee if apogee is lower.
    Below(f64),
}

/// One parachute, by its drag area.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Chute {
    /// The example's name for it.
    pub name: &'static str,
    /// Its drag area `C_D S`, m².
    pub cd_s_m2: f64,
    /// When it opens.
    pub opens: Opens,
    /// The delay from its trigger to its opening, s.
    pub lag_s: f64,
}

/// A design the flights draw from.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Design {
    /// Its file's stem under `validation/designs/`.
    pub name: &'static str,
    /// The configuration that flies.
    pub configuration: &'static str,
    /// Its parachutes in the order they open; each releases the one before, as RocketPy's
    /// parachutes replace one another.
    pub chutes: &'static [Chute],
}

/// RocketPy's Calisto, both examples: a 1 m² drogue at apogee and a 10 m² main at 800 m, each
/// after 1.5 s (`rocket_mass.py:204-207`, `:250-253`).
const CALISTO: &[Chute] = &[
    Chute {
        name: "Drogue",
        cd_s_m2: 1.0,
        opens: Opens::Apogee,
        lag_s: 1.5,
    },
    Chute {
        name: "Main",
        cd_s_m2: 10.0,
        opens: Opens::Below(800.0),
        lag_s: 1.5,
    },
];

/// The ten designs, each equally likely in a draw.
pub const DESIGNS: [Design; 10] = [
    Design {
        name: "rocketpy-bella-lui",
        configuration: "example",
        // `rocket_mass.py:305-306`.
        chutes: &[Chute {
            name: "Drogue",
            cd_s_m2: PI / 4.0,
            opens: Opens::Apogee,
            lag_s: 1.0,
        }],
    },
    Design {
        name: "rocketpy-calisto-getting-started-motor-at-minus-1.255",
        configuration: "example",
        chutes: CALISTO,
    },
    Design {
        name: "rocketpy-calisto-tests-motor-at-minus-1.373",
        configuration: "example",
        chutes: CALISTO,
    },
    Design {
        name: "rocketpy-cavour",
        configuration: "example",
        // The example has no parachute (`rocket_mass.py:482-485`): a flight whose recovery failed.
        chutes: &[],
    },
    Design {
        name: "rocketpy-genesis",
        configuration: "example",
        // `rocket_mass.py:582-585`.
        chutes: &[
            Chute {
                name: "Drogue",
                cd_s_m2: 0.285_005_285_533_666,
                opens: Opens::Apogee,
                lag_s: 1.0,
            },
            Chute {
                name: "Main",
                cd_s_m2: 1.1,
                opens: Opens::Below(870.0),
                lag_s: 1.0,
            },
        ],
    },
    Design {
        name: "rocketpy-juno-iii",
        configuration: "example",
        // `rocket_mass.py:476-477`.
        chutes: &[Chute {
            name: "Drogue",
            cd_s_m2: 0.885,
            opens: Opens::Apogee,
            lag_s: 0.5,
        }],
    },
    Design {
        name: "rocketpy-lince",
        configuration: "example",
        // `rocket_mass.py:639-640`: one main, at apogee, with no lag.
        chutes: &[Chute {
            name: "Main",
            cd_s_m2: 3.9,
            opens: Opens::Apogee,
            lag_s: 0.0,
        }],
    },
    Design {
        name: "rocketpy-ndrt-2020-nose-to-tail",
        configuration: "example",
        // `rocket_mass.py:360-365`: a 24 in drogue of C_D 1.5 and a 120 in main of C_D 2.2.
        chutes: &[
            Chute {
                name: "Drogue",
                cd_s_m2: 1.5 * PI * (24.0 * INCH_M) * (24.0 * INCH_M) / 4.0,
                opens: Opens::Apogee,
                lag_s: 1.0,
            },
            Chute {
                name: "Main",
                cd_s_m2: 2.2 * PI * (120.0 * INCH_M) * (120.0 * INCH_M) / 4.0,
                opens: Opens::Below(167.64),
                lag_s: 1.0,
            },
        ],
    },
    Design {
        name: "rocketpy-prometheus-2022-generic-motor",
        configuration: "example",
        // `rocket_mass.py:688-689`, with RocketPy's default lag of none.
        chutes: &[
            Chute {
                name: "Drogue",
                cd_s_m2: 1.6 * PI * FOOT_M * FOOT_M,
                opens: Opens::Apogee,
                lag_s: 0.0,
            },
            Chute {
                name: "Main",
                cd_s_m2: 2.2 * PI * (3.0 * FOOT_M) * (3.0 * FOOT_M),
                opens: Opens::Below(457.2),
                lag_s: 0.0,
            },
        ],
    },
    Design {
        name: "rocketpy-valetudo",
        configuration: "example",
        // `rocket_mass.py:417-418`.
        chutes: &[Chute {
            name: "Drogue",
            cd_s_m2: 0.349 * 1.3,
            opens: Opens::Apogee,
            lag_s: 1.0,
        }],
    },
];

/// The text of each design's file, in [`DESIGNS`]'s order, as the caller reads it from
/// `validation/designs/<name>.json`. The library does no I/O, and a published crate can't embed a
/// file outside its own folder, so the files come from whoever flies the flights.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignFiles(Vec<String>);

impl DesignFiles {
    /// The files' text, `files[k]` that of `DESIGNS[k]`.
    ///
    /// # Errors
    ///
    /// [`SyntheticError::Invalid`] unless there is one file for each design.
    pub fn new(files: Vec<String>) -> Result<Self, SyntheticError> {
        if files.len() == DESIGNS.len() {
            Ok(Self(files))
        } else {
            Err(SyntheticError::Invalid {
                what: "the design files aren't one for each of the ten designs",
            })
        }
    }

    /// The file of design `design`, an index into [`DESIGNS`].
    pub fn json(&self, design: usize) -> Option<&str> {
        self.0.get(design).map(String::as_str)
    }

    /// The files as the repository holds them, for the tests.
    #[cfg(test)]
    pub(crate) fn committed() -> Self {
        macro_rules! files {
            ($($name:literal),* $(,)?) => {
                vec![$(include_str!(concat!("../../../../validation/designs/", $name, ".json")).to_owned()),*]
            };
        }
        Self(files![
            "rocketpy-bella-lui",
            "rocketpy-calisto-getting-started-motor-at-minus-1.255",
            "rocketpy-calisto-tests-motor-at-minus-1.373",
            "rocketpy-cavour",
            "rocketpy-genesis",
            "rocketpy-juno-iii",
            "rocketpy-lince",
            "rocketpy-ndrt-2020-nose-to-tail",
            "rocketpy-prometheus-2022-generic-motor",
            "rocketpy-valetudo",
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_committed_files_are_the_designs_in_order() {
        let files = DesignFiles::committed();
        for (k, design) in DESIGNS.iter().enumerate() {
            let rocket: serde_json::Value = serde_json::from_str(files.json(k).unwrap()).unwrap();
            assert!(rocket.is_object(), "{}", design.name);
        }
        assert!(DesignFiles::new(vec![String::new(); 9]).is_err());
        assert!(files.json(DESIGNS.len()).is_none());
    }
}
