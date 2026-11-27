#!/usr/bin/env python3
"""Fetches the real flight logs in RocketPy's data that hpr's tests read, at the commit
`validation/refs.lock.toml` pins RocketPy to, checks each file's SHA-256 and writes them under the
folder given, laid out as RocketPy's `data/rockets` folder is. Every file is fetched and checked
before any is written, so a mismatch leaves the folder as it was.

A machine with RocketPy's checkout under `refs/` has them there; CI has no `refs/`, so its Linux
test jobs fetch these files and name the folder in `HPR_ROCKETPY_ROCKETS` and NDRT 2020's log in
`HPR_NDRT_2020_LOG`, which makes the tests that read them fail where they'd otherwise pass over
them (issues #1648, #1657). Its Python jobs fetch NDRT 2020's log alone, naming it after the
folder. Only these files are fetched: a partial checkout under `refs/rocketpy` would switch on
checks that need the whole of it.

Issue #1657 named two more tests that pass over a file CI lacks; this script doesn't fetch
their files, and each test says why in its doc comment: `analyze_reads_the_public_pnut_log_as_it_states`
(a log whose upstream terms are unclear, so it isn't fetched for CI) and `hpr-net`'s
`each_record_is_the_raw_snapshots_own` (ThrustCurve.org's answer as recorded on 2026-10-28, which a
search today needn't match).

The logs are RocketPy's (MIT; data files carry their own terms; THIRD-PARTY-NOTICES.md). They are
fetched, never committed.

Usage: python3 scripts/fetch-rocketpy-logs.py <out-folder> [<file under data/rockets> ...]
"""

import hashlib
import http.client
import pathlib
import sys
import time
import tomllib
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parent.parent
LOCK = ROOT / "validation" / "refs.lock.toml"
FOLDER = "data/rockets"
# Each file under FOLDER, and its SHA-256 at the pinned commit (RocketPy v1.13.0): the 22 logs
# `analyze_writes_a_ghost_of_each_of_rocketpys_logs` reads, NDRT 2020's and Prometheus's among them.
FILES = {
    "NDRT_2020/ndrt_2020_flight_data.csv": "5e64b446199988ac183b2f3965d76da27e57496bb2a33f78df73aa23fd18d7b8",
    "andromeda/flight_data.csv": "ebd3b757555b564c63937b69d2c222d6596355fc6e938d49537af293a9d48d14",
    "juno3/cots_altimeter.csv": "eef81b3bcc4c285667727c1e2c9ca502a67cb374f5e31bc4f87ed76b8a04c1ee",
    "prometheus/2022-06-24-serial-5115-flight-0001-TeleMetrum.csv": "9fe8d17a25e7944267b34a45ed955303afd5818b3f18479a5f7ffb31a574cecf",
    "prometheus/2022-06-24-serial-6583-flight-0003-TeleMega.csv": "befe40714e6b88b3bd5af8cd51b569a94544c486ced299878f66ab264f9105a6",
    "EPFL_Bella_Lui/bella_lui_flight_data_filtered.csv": "ccf8bcecd91d5b912e8f8b3bcf9efe13248453e1a81dc79d2b4daea4e985de2d",
    "astg/altimeter_halcyon.csv": "7f1fec2faa53f63a183c97d8a9b692763e7b53d60eab15cc6f42ab646afa85a0",
    "astg/gnss_halcyon.csv": "1a4715c45c85671c41fd8b3a870a9650548e7e5bae70c781c5e34f054090862f",
    "astra/flight_data.csv": "dd5287b375e84be8961a7c867dc9af764a42752af3109c25086d228978e4acae",
    "camoes/flight_data.csv": "8ceb1a196901569e32a6e9f09067feebb5ca1406d686e402f726e715a1be1dcc",
    "camoes/gnss_flight.csv": "ea6129167650b503477282e6dd2864530145bc33a26487a612dcfc4e781f99e6",
    "erebus11/flight_data_filtered.csv": "045f5b9ed473b80095805d3ca7b831ca4b67c5afbe9156d58d1e91df9fc0b4a9",
    "genesis/flight_data_faraday.csv": "d88a7fc53cbd55d1dafeed5d5c6fcfcad27c7cf8e0a15221e67aae97eaa05a74",
    "genesis/gnss_faraday.csv": "5843d20becfb8cda3246f7ebbca2e090579ffffd5ac148ae3519be91a0c7b90b",
    "hedy/cats_tust/filteredDataInfo.csv": "a5fe9e1a760a17a8c22c920097f3658e00b42030bca7f44123e20d5425fa0d9b",
    "hedy/cats_tust/flightInfo.csv": "481375091cf0da74fd1d933189546d9b846b3ca54b4c29f90d4a90f40b5f7731",
    "hedy/cats_tust/gnssInfo.csv": "8ab7e041385698a4adab7d9abbd3c985c5a38b42ead8667a5b53fc9223cbcbc3",
    "juno3/cots_GNSS.csv": "557c80be4c0d9a5e6a5493177a9adfaa3b02c11b8ff814ed238b53540ac9f062",
    "juno3/srad_telemetry.csv": "8cff1691d9a89bfccdb3a85f1a55ac26432b220c64e1c4c80cd7df16077af39c",
    "lince/main_data.csv": "a9cf47fe2b2e5be03168d36b1c41f0a5651a0c3b6db4ed091e1afc274bd7473e",
    "polito/altimeter_cavour.csv": "f912724676eea0ca9ba031c3779e48f861538814170b050b7bda2887c31f29a8",
    "valkyrie/flightInfo_merged.csv": "0c7808a84fb39c83908ba9caa1d2d3ccb609d6db46c3d660b85977e6917795bc",
}
ATTEMPTS = 3


def pinned_commit() -> str:
    with LOCK.open("rb") as lock:
        pins = tomllib.load(lock)
    for git in pins["git"]:
        if git["name"] == "rocketpy":
            return git["commit"]
    raise SystemExit(f"{LOCK} pins no repository named rocketpy")


def fetch(url: str) -> bytes:
    for attempt in range(1, ATTEMPTS + 1):
        try:
            with urllib.request.urlopen(url, timeout=60) as response:
                return response.read()
        except (OSError, http.client.HTTPException) as error:
            if attempt == ATTEMPTS:
                raise SystemExit(f"{url}: {error}") from error
            time.sleep(5 * attempt)
    raise AssertionError("unreachable: the last attempt returns or exits")


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__.strip().splitlines()[-1], file=sys.stderr)
        return 2
    out = pathlib.Path(sys.argv[1])
    names = sys.argv[2:] or list(FILES)
    unknown = [name for name in names if name not in FILES]
    if unknown:
        print(f"not among the files this script fetches: {', '.join(unknown)}", file=sys.stderr)
        return 2
    commit = pinned_commit()
    fetched = {}
    for name in names:
        url = f"https://raw.githubusercontent.com/RocketPy-Team/RocketPy/{commit}/{FOLDER}/{name}"
        data = fetch(url)
        digest = hashlib.sha256(data).hexdigest()
        if digest != FILES[name]:
            raise SystemExit(
                f"{url}: SHA-256 {digest}, expected {FILES[name]}. If {LOCK.name}'s rocketpy pin "
                f"moved, check the file and update FILES in {pathlib.Path(__file__).name}"
            )
        fetched[name] = data
    for name, data in fetched.items():
        path = out / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    total = sum(len(data) for data in fetched.values())
    print(f"{out}: {len(fetched)} files, {total} bytes, each SHA-256 checked")
    return 0


if __name__ == "__main__":
    sys.exit(main())
