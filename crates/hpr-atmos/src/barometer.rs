//! What a barometric altimeter reads in the day's air.

use crate::{AtmosError, Atmosphere, Ussa76};

/// A barometric altimeter on a pad, in the air of a day: it reads the standard atmosphere's
/// altitude of the day's pressure where it is, less that at the pad, `H(p(z)) − H(p(z₀))`
/// ([`Ussa76::pressure_altitude_m`]).
///
/// In the standard atmosphere itself it reads the geopotential height climbed. On a warmer day
/// the air is thinner, pressure falls more slowly with height, and it reads less than that: in
/// the troposphere, with the sea-level pressure unchanged, by the ratio of the standard's
/// sea-level temperature to the day's.
#[derive(Debug)]
pub struct Barometer<'a> {
    day: &'a dyn Atmosphere,
    site_msl_m: f64,
    standard: Ussa76,
    pad_altitude_m: f64,
}

impl<'a> Barometer<'a> {
    /// A barometer on a pad `pad_m` above a site `site_msl_m` above sea level, in `day`'s air.
    ///
    /// # Errors
    ///
    /// [`AtmosError`] if the day has no air at the pad.
    pub fn on_pad(
        day: &'a dyn Atmosphere,
        site_msl_m: f64,
        pad_m: f64,
    ) -> Result<Self, AtmosError> {
        let standard = Ussa76::standard();
        let pad_altitude_m =
            standard.pressure_altitude_m(day.air(site_msl_m + pad_m)?.air.pressure_pa)?;
        Ok(Self {
            day,
            site_msl_m,
            standard,
            pad_altitude_m,
        })
    }

    /// What it reads at `height_m` above the site, m.
    ///
    /// # Errors
    ///
    /// [`AtmosError`] if the day has no air there.
    pub fn reading_m(&self, height_m: f64) -> Result<f64, AtmosError> {
        let pressure_pa = self.day.air(self.site_msl_m + height_m)?.air.pressure_pa;
        Ok(self.standard.pressure_altitude_m(pressure_pa)? - self.pad_altitude_m)
    }

    /// The height above the site at which it reads `reading_m`, m: the inverse of
    /// [`Self::reading_m`], which turns a logged barometric altitude into the height the rocket
    /// climbed in the day's air ([the private collection's logged apogees, ADR-184][adr-184]).
    ///
    /// [adr-184]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0184-logged-apogees-of-the-private-collection.md
    ///
    /// The reading rises with height wherever the day's pressure falls with height, so the
    /// height is bisected between the site and the first doubling of the reading's size that
    /// reads past it, to the resolution of `f64`.
    ///
    /// # Errors
    ///
    /// [`AtmosError::Domain`] for a reading that is not finite, below what the site
    /// itself reads, or above what the day reads 100 km up; and what [`Self::reading_m`] raises.
    pub fn height_m(&self, reading_m: f64) -> Result<f64, AtmosError> {
        const TOP_M: f64 = 100_000.0;
        let domain = || AtmosError::Domain {
            what: "barometric reading",
            value: reading_m,
        };
        if !reading_m.is_finite() {
            return Err(domain());
        }
        let mut low = 0.0;
        if self.reading_m(low)? > reading_m {
            return Err(domain());
        }
        let mut high = (2.0 * reading_m.abs()).clamp(100.0, TOP_M);
        while self.reading_m(high)? < reading_m {
            if high >= TOP_M {
                return Err(domain());
            }
            low = high;
            high = (2.0 * high).min(TOP_M);
        }
        loop {
            let middle = 0.5 * (low + high);
            if middle <= low || middle >= high {
                return Ok(middle);
            }
            if self.reading_m(middle)? < reading_m {
                low = middle;
            } else {
                high = middle;
            }
        }
    }
}
