//! Foreign formats: designs, parts, weather and elevation.
//!
//! [Start here][guide-start] says what works today and what is planned.
//!
//! [guide-start]: https://hpr.fusionspace.co/start-here.html
//! [roadmap]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md
//!
//! Status: pre-alpha. What each module reads:
//!
//! - [`ork`]: an OpenRocket `.ork` file, read into a design, its motors and recovery.
//! - [`orc`]: OpenRocket's `.orc` parts catalogs, read into parts by maker and part number. The
//!   16 files OpenRocket 24.12 ships are built in ([parts catalogs][guide-orc]).
//! - [`era5`]: an ERA5 pressure-level file, which gives the atmosphere over a launch site
//!   ([ERA5 weather files][guide-era5]). It's read by the netCDF classic reader ([`netcdf`]).
//! - [`grib2`]: the GRIB2 fields NOAA's NOMADS cuts from its GFS and RAP forecasts. It also
//!   decodes whole GFS files' complex packing and JPEG 2000.
//! - [`geotiff`]: a launch site's height from a GeoTIFF elevation file ([A launch site's
//!   elevation][guide-elevation]).
//!
//! The other design formats, RockSim's `.rkt`, RASAero's `.CDX1` and RocketPy's export, are
//! planned ([the roadmap][roadmap]).
//!
//! [guide-era5]: https://hpr.fusionspace.co/format/era5.html
//! [guide-orc]: https://hpr.fusionspace.co/format/orc.html
//! [guide-elevation]: https://hpr.fusionspace.co/elevation.html#from-an-elevation-file-of-your-own

pub mod era5;
pub mod geotiff;
pub mod grib2;
pub mod netcdf;
pub mod orc;
pub mod ork;
