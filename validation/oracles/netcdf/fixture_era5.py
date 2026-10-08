"""The ERA5 pressure-level files of the private flight collection, converted for hpr's reader.

`hpr-sim-fixtures` (ADR-151) holds, for each tier-A flight, the ERA5 pressure-level file of its
day as the current Climate Data Store writes it: netCDF-4, which `hpr_io::netcdf` refuses and
names the conversion for. M2.3c1 (ADR-184) flies each flight in that weather, so this script runs
the guide's conversion, word for word, on every tier-A flight's pressure-level file:

    xarray.open_dataset(source).drop_vars(["number", "expver"], errors="ignore")
        .to_netcdf(out, format="NETCDF3_64BIT")

and writes each to `corpus-out/hpr-sim-fixtures/era5/<flight id>/<file name>`, beside an
`index.json` that records each source's and output's SHA-256. Everything it writes stays under the
gitignored `corpus-out/`: the files are the collection's (ERA5 itself may be redistributed with
attribution, but the flights' ids and sites are private). Nothing is written when the collection
is not checked out.

Run from the repository root with the oracle environment:

    refs/venv/bin/python validation/oracles/netcdf/fixture_era5.py
"""

import csv
import hashlib
import json
import sys
from pathlib import Path

import xarray

COLLECTION = Path("refs/hpr-sim-fixtures")
OUT = Path("corpus-out/hpr-sim-fixtures/era5")


def sha256(path):
    digest = hashlib.sha256()
    digest.update(path.read_bytes())
    return digest.hexdigest()


def main():
    # The collection is a symlink to a sibling checkout; resolve it rather than skip it (#306).
    root = COLLECTION.resolve()
    if not (root / "flights.csv").is_file():
        sys.exit(f"{COLLECTION} is not checked out")
    with open(root / "flights.csv", newline="", encoding="utf-8") as f:
        tier_a = sorted(row["flight_id"] for row in csv.DictReader(f) if row["tier"] == "A")
    index = []
    for flight in tier_a:
        folder = root / "flights" / flight / "weather" / "era5"
        for source in sorted(folder.glob("*pressure-levels*.nc")):
            out = OUT / flight / source.name
            out.parent.mkdir(parents=True, exist_ok=True)
            with xarray.open_dataset(source) as ds:
                ds.drop_vars(["number", "expver"], errors="ignore").to_netcdf(
                    out, format="NETCDF3_64BIT"
                )
            index.append(
                {
                    "flight_id": flight,
                    "source": str(COLLECTION / "flights" / flight / "weather" / "era5" / source.name),
                    "source_sha256": sha256(source),
                    "converted": str(out),
                    "converted_sha256": sha256(out),
                }
            )
    (OUT / "index.json").write_text(json.dumps({"files": index}, indent=1) + "\n")
    print(f"{len(index)} files of {len(tier_a)} tier-A flights converted", file=sys.stderr)


if __name__ == "__main__":
    main()
