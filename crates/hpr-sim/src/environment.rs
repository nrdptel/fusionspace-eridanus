//! The flight's surroundings: the Earth model with the launch site, the atmosphere and the wind.

use std::sync::Arc;

use hpr_atmos::{AirState, Atmosphere, AtmosphereModel, ConstantWind, Wind};
use hpr_core::DVec3;
use hpr_core::earth::Earth;
use hpr_core::geodesy::Geodetic;

use crate::error::SimError;

/// The Earth, atmosphere and wind a flight sees.
///
/// Heights: the state and the Earth model use the launch frame and ellipsoidal heights. The
/// atmosphere and wind take height above mean sea level, `H = h − N`, with the geoid undulation
/// `N` at the site given here (hpr has no geoid model; `docs/physics/geodesy.md`). The ground is
/// the ellipsoidal height of the site. Wind vectors are taken in the launch frame's axes.
#[derive(Debug, Clone)]
pub struct Environment {
    /// The Earth: the launch site and frame, gravity and rotation.
    pub earth: Earth,
    /// The geoid undulation `N` at the site, m (`h = H + N`).
    pub geoid_undulation_m: f64,
    /// The atmosphere, by height above mean sea level. Shared, so environments clone cheaply
    /// across threads.
    pub atmosphere: Arc<dyn Atmosphere>,
    /// The wind, by height above mean sea level, in launch-frame axes.
    pub wind: Arc<dyn Wind>,
}

impl Environment {
    /// An environment from its parts, with `N = 0`.
    pub fn new(
        earth: Earth,
        atmosphere: impl Atmosphere + 'static,
        wind: impl Wind + 'static,
    ) -> Self {
        Self {
            earth,
            geoid_undulation_m: 0.0,
            atmosphere: Arc::new(atmosphere),
            wind: Arc::new(wind),
        }
    }

    /// WGS 84 at `site` (ellipsoidal height), the 1976 standard atmosphere and no wind.
    ///
    /// # Errors
    ///
    /// [`SimError::Core`] for an invalid site.
    pub fn standard(site: Geodetic) -> Result<Self, SimError> {
        Ok(Self::new(
            Earth::wgs84(site)?,
            AtmosphereModel::default(),
            ConstantWind::calm(),
        ))
    }

    /// The same environment with geoid undulation `N`, m.
    #[must_use]
    pub fn with_geoid_undulation_m(mut self, undulation_m: f64) -> Self {
        self.geoid_undulation_m = undulation_m;
        self
    }

    /// The same environment with `wind` in place of its wind.
    #[must_use]
    pub fn with_wind(mut self, wind: impl Wind + 'static) -> Self {
        self.wind = Arc::new(wind);
        self
    }

    /// The launch site.
    #[must_use]
    pub fn site(&self) -> Geodetic {
        self.earth.frame().origin()
    }

    /// The wind's velocity at `height_msl_m` above mean sea level, in the launch frame's
    /// East-North-Up axes, m/s. Every place the flight reads the wind reads it here, so a wind of
    /// a program's own that returns a velocity that is not finite (NaN or infinite) is refused
    /// where it is read, naming the wind and the height, rather than surfacing later as some other
    /// quantity that isn't finite (issue #237).
    ///
    /// # Errors
    ///
    /// [`SimError::Atmosphere`] if the wind refuses the height; [`SimError::Domain`] with
    /// [`WIND_NOT_FINITE`] and the height above mean sea level, m, if any component of the
    /// velocity is not finite.
    pub(crate) fn wind_enu_m_s(&self, height_msl_m: f64) -> Result<DVec3, SimError> {
        let velocity_enu_m_s = self.wind.wind(height_msl_m)?.velocity_enu_m_s;
        if velocity_enu_m_s.is_finite() {
            Ok(velocity_enu_m_s)
        } else {
            Err(SimError::Domain {
                what: WIND_NOT_FINITE,
                value: height_msl_m,
            })
        }
    }

    /// The air at `height_msl_m` above mean sea level. Every place the flight reads the air reads
    /// it here, so an atmosphere of a program's own that returns air the flight can't use is
    /// refused where it is read, naming the field and the height, rather than flying on with a
    /// wrong number (issue #301). Before, a density that was NaN or negative turned the drag off
    /// without an error: a separated body landed at about 140 m/s, a climb went twice as high.
    ///
    /// Every field must be finite. The temperature, speed of sound and viscosity must also be
    /// positive. The density and pressure may be zero, as in a vacuum or far above the 1976
    /// standard atmosphere's top, where both shrink toward zero, but not negative. The air is
    /// returned unchanged, so good air flies exactly as it did before the check.
    ///
    /// # Errors
    ///
    /// [`SimError::Atmosphere`] if the atmosphere refuses the height; [`SimError::Domain`] with
    /// the field's constant ([`AIR_DENSITY_REFUSED`], [`AIR_PRESSURE_REFUSED`],
    /// [`AIR_TEMPERATURE_REFUSED`], [`AIR_SPEED_OF_SOUND_REFUSED`] or
    /// [`AIR_VISCOSITY_REFUSED`]) and the height above mean sea level, m, for the first field,
    /// in that order, the flight can't use.
    pub(crate) fn air_at(&self, height_msl_m: f64) -> Result<AirState, SimError> {
        let air = self.atmosphere.air(height_msl_m)?.air;
        let not_negative = |x: f64| x.is_finite() && x >= 0.0;
        let positive = |x: f64| x.is_finite() && x > 0.0;
        let refused = if !not_negative(air.density_kg_m3) {
            AIR_DENSITY_REFUSED
        } else if !not_negative(air.pressure_pa) {
            AIR_PRESSURE_REFUSED
        } else if !positive(air.temperature_k) {
            AIR_TEMPERATURE_REFUSED
        } else if !positive(air.speed_of_sound_m_s) {
            AIR_SPEED_OF_SOUND_REFUSED
        } else if !positive(air.dynamic_viscosity_pa_s) {
            AIR_VISCOSITY_REFUSED
        } else {
            return Ok(air);
        };
        Err(SimError::Domain {
            what: refused,
            value: height_msl_m,
        })
    }
}

/// What a [`SimError::Domain`] names when the wind's velocity is not finite; its value is the
/// height above mean sea level, m, at which the wind was read.
pub(crate) const WIND_NOT_FINITE: &str =
    "height above sea level, m, at which the wind's velocity is not finite";

/// What a [`SimError::Domain`] names when the air's density is negative or not finite; its value
/// is the height above mean sea level, m, at which the air was read.
pub(crate) const AIR_DENSITY_REFUSED: &str =
    "height above sea level, m, at which the air's density is negative or not finite";

/// What a [`SimError::Domain`] names when the air's pressure is negative or not finite; its value
/// is the height above mean sea level, m, at which the air was read.
pub(crate) const AIR_PRESSURE_REFUSED: &str =
    "height above sea level, m, at which the air's pressure is negative or not finite";

/// What a [`SimError::Domain`] names when the air's temperature is zero, negative or not finite;
/// its value is the height above mean sea level, m, at which the air was read.
pub(crate) const AIR_TEMPERATURE_REFUSED: &str =
    "height above sea level, m, at which the air's temperature is zero, negative or not finite";

/// What a [`SimError::Domain`] names when the air's speed of sound is zero, negative or not
/// finite; its value is the height above mean sea level, m, at which the air was read.
pub(crate) const AIR_SPEED_OF_SOUND_REFUSED: &str = "height above sea level, m, at which the \
     air's speed of sound is zero, negative or not finite";

/// What a [`SimError::Domain`] names when the air's dynamic viscosity is zero, negative or not
/// finite; its value is the height above mean sea level, m, at which the air was read.
pub(crate) const AIR_VISCOSITY_REFUSED: &str = "height above sea level, m, at which the air's \
     viscosity is zero, negative or not finite";

#[cfg(test)]
mod tests {
    use hpr_atmos::ConstantWind;

    use std::sync::Arc;

    use hpr_atmos::Atmosphere;

    use super::Environment;
    use crate::testing::site;

    #[test]
    fn with_wind_replaces_only_the_wind() {
        let calm = Environment::standard(site()).unwrap();
        // 5 m/s from the west blows toward the east.
        let windy = calm
            .clone()
            .with_wind(ConstantWind::new(5.0, 1.5 * std::f64::consts::PI).unwrap());
        let at =
            |environment: &Environment| environment.wind.wind(1500.0).unwrap().velocity_enu_m_s;
        assert_eq!(at(&calm).length(), 0.0);
        assert!((at(&windy).x - 5.0).abs() < 1e-12 && at(&windy).y.abs() < 1e-12);
        assert_eq!(windy.site(), calm.site());
        assert_eq!(windy.geoid_undulation_m, calm.geoid_undulation_m);
        assert_eq!(
            windy.atmosphere.air(1500.0).unwrap(),
            calm.atmosphere.air(1500.0).unwrap()
        );
    }

    /// The check on the air refuses nothing hpr's own atmospheres return, from 5 km below sea
    /// level to a million kilometers up, where the standard's pressure and density have shrunk to
    /// zero: the standard, offset as far as it allows either way, a humid sounding extrapolated
    /// past both ends, and the tests' vacuum. It returns the air unchanged.
    #[test]
    fn the_check_on_the_air_takes_every_stock_atmosphere() {
        use hpr_atmos::{AtmosphereModel, Ussa76};

        use crate::testing::UniformAir;

        let sounding: AtmosphereModel = serde_json::from_str(
            r#"{"model":"sounding","latitude_rad":0.6,"levels":[
            {"height_msl_m":1400.0,"temperature_k":300.0,"pressure_pa":85000.0,
             "relative_humidity":1.0},
            {"height_msl_m":3000.0,"temperature_k":290.0,"pressure_pa":70000.0,
             "relative_humidity":0.5}]}"#,
        )
        .unwrap();
        let atmospheres: [Arc<dyn Atmosphere>; 5] = [
            Arc::new(Ussa76::standard()),
            Arc::new(Ussa76::with_offset(-186.0, 30_000.0).unwrap()),
            Arc::new(Ussa76::with_offset(60.0, 110_000.0).unwrap()),
            Arc::new(sounding),
            Arc::new(UniformAir::vacuum()),
        ];
        // Every 500 m from 5 km below sea level to 1 km above it, then 25% higher each time, and
        // last a million kilometers up.
        let mut heights_msl_m: Vec<f64> = (-10..=2).map(|i| f64::from(i) * 500.0).collect();
        while let Some(&last) = heights_msl_m.last().filter(|&&h| h < 1.0e9) {
            heights_msl_m.push((last * 1.25).min(1.0e9));
        }
        assert_eq!(heights_msl_m.last(), Some(&1.0e9));
        for atmosphere in atmospheres {
            let mut environment = Environment::standard(site()).unwrap();
            environment.atmosphere = atmosphere;
            for &height_msl_m in &heights_msl_m {
                let air = environment.air_at(height_msl_m).unwrap();
                assert_eq!(
                    air,
                    environment.atmosphere.air(height_msl_m).unwrap().air,
                    "{height_msl_m}"
                );
            }
        }
        // The standard's own air at the top of the sweep has a density and pressure of zero, so
        // the check must take a zero.
        let far = Environment::standard(site())
            .unwrap()
            .air_at(1.0e9)
            .unwrap();
        assert_eq!((far.density_kg_m3, far.pressure_pa), (0.0, 0.0));
    }

    /// Each field the flight can't use is refused by name, with the height; the first such field,
    /// in the order density, pressure, temperature, speed of sound, viscosity, is the one named.
    #[test]
    fn the_check_on_the_air_names_the_field_and_the_height() {
        use super::{
            AIR_DENSITY_REFUSED, AIR_PRESSURE_REFUSED, AIR_SPEED_OF_SOUND_REFUSED,
            AIR_TEMPERATURE_REFUSED, AIR_VISCOSITY_REFUSED,
        };
        use crate::error::SimError;
        use crate::testing::UniformAir;

        let refusals = [
            (AIR_DENSITY_REFUSED, true),
            (AIR_PRESSURE_REFUSED, true),
            (AIR_TEMPERATURE_REFUSED, false),
            (AIR_SPEED_OF_SOUND_REFUSED, false),
            (AIR_VISCOSITY_REFUSED, false),
        ];
        let set = |air: &mut hpr_atmos::AirState, field: usize, value: f64| {
            *[
                &mut air.density_kg_m3,
                &mut air.pressure_pa,
                &mut air.temperature_k,
                &mut air.speed_of_sound_m_s,
                &mut air.dynamic_viscosity_pa_s,
            ]
            .into_iter()
            .nth(field)
            .unwrap() = value;
        };
        let check = |air: hpr_atmos::AirState| {
            let mut environment = Environment::standard(site()).unwrap();
            environment.atmosphere = Arc::new(UniformAir(air));
            environment.air_at(1234.5)
        };
        for (field, (refusal, takes_zero)) in refusals.into_iter().enumerate() {
            let mut bad = vec![
                f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY,
                -1.0,
                -f64::MIN_POSITIVE,
            ];
            if !takes_zero {
                bad.extend([0.0, -0.0]);
            }
            for value in bad {
                // Every later field spoilt too: the first is the one named.
                let mut air = UniformAir::sea_level().0;
                for later in field..5 {
                    set(&mut air, later, value);
                }
                match check(air) {
                    Err(SimError::Domain {
                        what,
                        value: height,
                    }) => {
                        assert_eq!(what, refusal, "{field} {value}");
                        assert_eq!(height, 1234.5, "{field} {value}");
                    }
                    other => panic!("{field} {value}: {other:?}"),
                }
            }
            let mut tiny = UniformAir::sea_level().0;
            set(&mut tiny, field, f64::MIN_POSITIVE);
            assert_eq!(check(tiny).unwrap(), tiny, "{field}");
            if takes_zero {
                for zero in [0.0, -0.0] {
                    let mut air = UniformAir::sea_level().0;
                    set(&mut air, field, zero);
                    assert_eq!(check(air).unwrap(), air, "{field} {zero}");
                }
            }
        }
    }
}
