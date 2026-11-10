"""EuRoC 2022's ERA5 file, converted for hpr's reader, and the checks that the conversion kept it.

Astra, Andromeda and Erebus 11 (EuRoC 2022) fly in `euroc_2022_all_windows.nc` from RocketPy
v1.13.0's `data/weather/` (`refs/rocketpy`). The file is netCDF-4, as the current Climate Data
Store writes it, which `hpr_io::netcdf` refuses and names the conversion for. This script runs
that conversion, word for word as the guide gives it (`docs/format/era5.md`):

    xarray.open_dataset(source).drop_vars(["number", "expver"], errors="ignore")
        .to_netcdf(out, format="NETCDF3_64BIT")

and writes `refs/converted/rocketpy/euroc_2022_all_windows.nc`. The converted file stays under
the gitignored `refs/`, as the source does (the weather file came to RocketPy with its flights'
data, whose terms are RocketPy's teams'); only the two files' SHA-256 are committed, in the JSON
this script prints.

It then tests the converted file, and fails unless every check holds:

1. it is a 64-bit offset file (`CDF\\x02`), and the only variables it lacks are `number` and
   `expver`, the two labels the classic formats cannot hold;
2. `z`, `t`, `u` and `v` hold the source's values bit for bit, on the same axes, and the
   latitudes, longitudes, pressure levels and times are the source's;
3. RocketPy 1.13.0's `Environment` (`type="Reanalysis"`, `dictionary="ECMWF"`), at each of the
   three flights' site and launch hour, builds the same levels from the converted file as from the
   source, bit for bit: pressure, geometric height, temperature and both wind components.

The launch hours are the notebooks' local times (`timezone="Portugal"`, Western European Summer
Time in October 2022, UTC+1) in UTC: `docs/examples/astra_flight_sim.ipynb:123-131`,
`andromeda_flight_sim.ipynb:123-131` and `erebus_flight_sim.ipynb:123-131`.

Run from the repository root with the oracle environment (`--check` writes no JSON and fails
unless the committed JSON's hashes are the ones this run finds):

    refs/venv/bin/python validation/oracles/netcdf/euroc_2022.py \\
        > validation/fixtures/weather/euroc-2022-conversion.json

    refs/venv/bin/python validation/oracles/netcdf/euroc_2022.py --check
"""

import datetime
import hashlib
import importlib.metadata
import json
import sys
import warnings
from pathlib import Path

import netCDF4
import numpy as np
import xarray
from rocketpy import Environment

SOURCE = Path("refs/rocketpy/data/weather/euroc_2022_all_windows.nc")
OUT = Path("refs/converted/rocketpy/euroc_2022_all_windows.nc")
RECORD = Path("validation/fixtures/weather/euroc-2022-conversion.json")
# The date of the committed run: change it when the record is regenerated.
GENERATED = "2026-11-10"
COMMAND = (
    "refs/venv/bin/python validation/oracles/netcdf/euroc_2022.py "
    "> validation/fixtures/weather/euroc-2022-conversion.json"
)
DROPPED = ["number", "expver"]
FIELDS = ["z", "t", "u", "v"]

# Site (degrees, m above sea level) and launch hour (UTC) of each flight, from its notebook.
FLIGHTS = [
    {
        "id": "astra",
        "latitude_deg": 39.3897,
        "longitude_deg": -8.2889639,
        "elevation_m": 160,
        "utc": (2022, 10, 13, 15),
    },
    {
        "id": "andromeda",
        "latitude_deg": 39.388692,
        "longitude_deg": -8.287814,
        "elevation_m": 160,
        "utc": (2022, 10, 14, 13),
    },
    {
        "id": "erebus-11",
        "latitude_deg": 39.3897,
        "longitude_deg": -8.2889639,
        "elevation_m": 160,
        "utc": (2022, 10, 15, 15),
    },
]


def fail(message):
    sys.exit(f"euroc_2022.py: {message}")


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def convert(source, out):
    """The guide's conversion, word for word."""
    out.parent.mkdir(parents=True, exist_ok=True)
    xarray.open_dataset(source).drop_vars(DROPPED, errors="ignore").to_netcdf(
        out, format="NETCDF3_64BIT"
    )


def raw(dataset, name):
    """A variable's stored values, with no masking or scaling."""
    variable = dataset.variables[name]
    variable.set_auto_maskandscale(False)
    return np.asarray(variable[:])


def unix_seconds(dataset, name):
    """A time axis as whole seconds since 1970-01-01T00:00Z, whatever its units."""
    variable = dataset.variables[name]
    variable.set_auto_maskandscale(False)
    calendar = getattr(variable, "calendar", "standard")
    epoch = datetime.datetime(1970, 1, 1)
    out = []
    for when in netCDF4.num2date(variable[:], variable.units, calendar):
        seconds = (
            datetime.datetime(when.year, when.month, when.day, when.hour, when.minute, when.second)
            - epoch
        ).total_seconds()
        out.append(int(seconds))
    return out


def same_values(source, out):
    """Checks 1 and 2; returns what was compared, for the record."""
    if out.read_bytes()[:4] != b"CDF\x02":
        fail(f"{out} is not a 64-bit offset netCDF file")
    with netCDF4.Dataset(source) as before, netCDF4.Dataset(out) as after:
        lacking = sorted(set(before.variables) - set(after.variables))
        if lacking != sorted(DROPPED):
            fail(f"the conversion dropped {lacking}, not {sorted(DROPPED)}")
        added = sorted(set(after.variables) - set(before.variables))
        if added:
            fail(f"the conversion added {added}")
        values = 0
        for name in FIELDS:
            a, b = raw(before, name), raw(after, name)
            if before.variables[name].dimensions != after.variables[name].dimensions:
                fail(f"{name}'s axes changed")
            if a.dtype != np.float32 or b.dtype != np.float32 or a.shape != b.shape:
                fail(f"{name} is {b.dtype}{b.shape}, from {a.dtype}{a.shape}")
            if a.tobytes() != b.astype(a.dtype.newbyteorder("=")).tobytes():
                fail(f"{name}'s values changed")
            if not np.isfinite(b).all():
                fail(f"{name} holds a value that is not finite")
            for attribute in ("scale_factor", "add_offset"):
                if attribute in after.variables[name].ncattrs():
                    fail(f"{name} was packed ({attribute})")
            values += int(a.size)
        for name in ("latitude", "longitude", "pressure_level"):
            a, b = raw(before, name), raw(after, name)
            if a.shape != b.shape or not np.array_equal(a.astype(np.float64), b.astype(np.float64)):
                fail(f"the {name} axis changed")
            if before.variables[name].units != after.variables[name].units:
                fail(f"the {name} axis's units changed")
        times_before = unix_seconds(before, "valid_time")
        times_after = unix_seconds(after, "valid_time")
        if times_before != times_after:
            fail("the times changed")
        return {
            "field_values_identical": values,
            "times": len(times_after),
            "pressure_levels": int(raw(after, "pressure_level").size),
            "first_time_unix_s": times_after[0],
            "last_time_unix_s": times_after[-1],
        }, times_after


def rocketpy_levels(path, flight):
    """RocketPy's levels at a flight's site and hour, as arrays."""
    env = Environment(
        latitude=flight["latitude_deg"],
        longitude=flight["longitude_deg"],
        elevation=flight["elevation_m"],
        date=flight["utc"],
    )
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        env.set_atmospheric_model(type="Reanalysis", file=str(path), dictionary="ECMWF")
    functions = (env.pressure, env.temperature, env.wind_velocity_x, env.wind_velocity_y)
    return [np.asarray(f.source, dtype=np.float64) for f in functions]


def same_reading(source, out, times):
    """Check 3; returns each flight's hour and how many levels were compared."""
    readings = []
    for flight in FLIGHTS:
        year, month, day, hour = flight["utc"]
        unix_s = int(
            (datetime.datetime(year, month, day, hour) - datetime.datetime(1970, 1, 1)).total_seconds()
        )
        if unix_s not in times:
            fail(f"{flight['id']}: the file has no {year}-{month:02}-{day:02}T{hour:02}:00Z")
        before, after = rocketpy_levels(source, flight), rocketpy_levels(out, flight)
        for a, b in zip(before, after):
            if a.shape != b.shape or a.tobytes() != b.tobytes():
                fail(f"{flight['id']}: RocketPy reads the converted file differently")
        readings.append({
            "id": flight["id"],
            "utc": f"{year}-{month:02}-{day:02}T{hour:02}:00Z",
            "levels_identical": int(before[0].shape[0]),
        })
    return readings


def main():
    args = sys.argv[1:]
    if args not in ([], ["--check"]):
        fail(f"unknown arguments {args}")
    if not SOURCE.is_file():
        fail(f"{SOURCE} is not there: cargo xtask refs fetch rocketpy")
    if SOURCE.read_bytes()[:4] != b"\x89HDF":
        fail(f"{SOURCE} is not a netCDF-4 (HDF5) file")
    convert(SOURCE, OUT)
    checks, times = same_values(SOURCE, OUT)
    readings = same_reading(SOURCE, OUT, times)
    record = {
        "generator": "validation/oracles/netcdf/euroc_2022.py",
        "command": COMMAND,
        "generated": GENERATED,
        "conversion": (
            'xarray.open_dataset(source).drop_vars(["number", "expver"], errors="ignore")'
            '.to_netcdf(out, format="NETCDF3_64BIT")'
        ),
        "packages": {
            name: importlib.metadata.version(name)
            for name in ("xarray", "netCDF4", "numpy", "rocketpy")
        },
        "source": {"path": str(SOURCE), "sha256": sha256(SOURCE)},
        "converted": {"path": str(OUT), "sha256": sha256(OUT)},
        "dropped": DROPPED,
        "checks": checks,
        "rocketpy_readings": readings,
    }
    if args == ["--check"]:
        committed = json.loads(RECORD.read_text())
        for key in ("source", "converted"):
            if committed[key] != record[key]:
                fail(f"{RECORD}'s {key} is {committed[key]}, and this run's is {record[key]}")
        print(f"euroc_2022.py: {OUT} is the committed conversion", file=sys.stderr)
    else:
        print(json.dumps(record, indent=2))


main()
