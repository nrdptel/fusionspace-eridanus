"""ERA5 cuts for the descent-wind report's public team logs (issue #1605).

The descent-wind report (`cargo xtask descent-wind`) compares the wind read from a GPS-tracked
descent with ERA5's at the launch site and hour. RocketPy ships ERA5 files for its own example
logs; for the team logs read from their own repositories, this script makes the files.

For each case it:

1. takes the Climate Data Store's netCDF-4 download, requested as `REQUEST` gives (the dataset
   *ERA5 hourly data on pressure levels from 1940 to present*, the four variables HPR Sim reads,
   the levels from 1000 to 500 hPa, the hours around the launch and an area at least a quarter of
   a degree beyond the site on each side), from `refs/era5/descent-wind/<case>.nc`;
2. cuts the grid points around the site (the bracketing latitudes and longitudes and one more on
   each side, every time and level) and writes them in 64-bit offset netCDF, the conversion the
   format page gives (`drop_vars(["number", "expver"])`, `format="NETCDF3_64BIT"`), under
   `validation/fixtures/weather/era5/descent-wind-<case>.nc`, with the credit in its `comment`;
3. prints a record of the request and both files' SHA-256.

The cuts contain modified Copernicus Climate Change Service information (2026): ERA5 hourly data
on pressure levels (Hersbach et al., 2023, doi:10.24381/cds.bd0915c6), CC BY 4.0.

The download needs a Data Store account and the `cdsapi` package, which the oracle environment
doesn't carry; with `--fetch` the script requests each missing file itself. Run from the
repository root:

    python validation/oracles/netcdf/descent_wind_era5.py [--fetch] \
        > validation/fixtures/weather/descent-wind-era5.json
"""

import hashlib
import json
import os
import sys

import numpy as np
import xarray

DOWNLOADS = "refs/era5/descent-wind"
OUT = "validation/fixtures/weather/era5"
ATTRIBUTION = (
    "Contains modified Copernicus Climate Change Service information 2026 (ERA5 hourly data on "
    "pressure levels, doi:10.24381/cds.bd0915c6, CC BY 4.0), cut from a Climate Data Store "
    "download by validation/oracles/netcdf/descent_wind_era5.py."
)
LEVELS = ["500", "550", "600", "650", "700", "750", "775", "800", "825", "850", "875", "900",
          "925", "950", "975", "1000"]

# Each team log's site (from its GPS log), and the day and hours around its launch, in UTC.
CASES = [
    {
        "case": "itzamna",
        "site": (33.49817, -99.33435),
        "request": {"year": ["2026"], "month": ["03"], "day": ["30"],
                    "time": ["13:00", "14:00"], "area": [33.75, -99.75, 33.25, -99.0]},
    },
    {
        "case": "rit-testbed-2",
        "site": (42.70439, -77.19376),
        "request": {"year": ["2026"], "month": ["05"], "day": ["16"],
                    "time": ["17:00", "18:00", "19:00"], "area": [43.0, -77.5, 42.25, -76.75]},
    },
]


def request(case):
    """The Data Store request for a case."""
    return {
        "product_type": ["reanalysis"],
        "variable": ["geopotential", "temperature", "u_component_of_wind",
                     "v_component_of_wind"],
        "pressure_level": LEVELS,
        "data_format": "netcdf",
        "download_format": "unarchived",
        **case["request"],
    }


def sha256(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def window(values, x):
    """Indices of the two grid values around x and one more on each side, in the file's order."""
    order = np.argsort(values)
    sorted_values = values[order]
    i = int(np.searchsorted(sorted_values, x))
    if i == 0 or i == len(sorted_values):
        raise SystemExit(f"{x} is outside the grid {sorted_values}")
    low, high = max(i - 2, 0), min(i + 2, len(sorted_values))
    return sorted(order[low:high].tolist())


def main():
    fetch = "--fetch" in sys.argv[1:]
    os.makedirs(DOWNLOADS, exist_ok=True)
    records = []
    for case in CASES:
        source = os.path.join(DOWNLOADS, f"{case['case']}.nc")
        if not os.path.exists(source):
            if not fetch:
                raise SystemExit(f"{source} is missing: run with --fetch")
            import cdsapi

            cdsapi.Client(quiet=True).retrieve(
                "reanalysis-era5-pressure-levels", request(case), source)
        ds = xarray.open_dataset(source)
        lat, lon = case["site"]
        ds = ds.isel(latitude=window(ds["latitude"].values, lat),
                     longitude=window(ds["longitude"].values, lon))
        ds.attrs["comment"] = ATTRIBUTION
        out = os.path.join(OUT, f"descent-wind-{case['case']}.nc")
        ds.drop_vars(["number", "expver"], errors="ignore").to_netcdf(
            out, format="NETCDF3_64BIT")
        records.append({
            "case": case["case"],
            "site": list(case["site"]),
            "request": request(case),
            "download_sha256": sha256(source),
            "cut": out,
            "cut_sha256": sha256(out),
            "latitudes": [float(v) for v in ds["latitude"].values],
            "longitudes": [float(v) for v in ds["longitude"].values],
            "times": [str(v) for v in ds["valid_time"].values],
        })
    json.dump({"script_sha256": sha256(__file__), "attribution": ATTRIBUTION, "cases": records},
              sys.stdout, indent=1)
    print()


if __name__ == "__main__":
    main()
