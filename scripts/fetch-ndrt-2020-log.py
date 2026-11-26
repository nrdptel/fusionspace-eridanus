#!/usr/bin/env python3
"""Fetches NDRT 2020's Featherweight Raven log from RocketPy's data, at the commit
`validation/refs.lock.toml` pins RocketPy to, checks its SHA-256 and writes it to the path given.

The public-flight tests of `hpr fit`, `hpr compare` and Python's `FlightLog.fit` read this log.
A machine with RocketPy's checkout under `refs/` has it there; CI has no `refs/`, so it fetches
this one file and names it in `HPR_NDRT_2020_LOG`, which makes the tests fail where they'd
otherwise pass over the flight (issue #1648). Only this file is fetched: a partial checkout under
`refs/rocketpy` would switch on checks that need the whole of it.

The log is RocketPy's (MIT; THIRD-PARTY-NOTICES.md). It is fetched, never committed.

Usage: python3 scripts/fetch-ndrt-2020-log.py <out.csv>
"""

import hashlib
import pathlib
import sys
import time
import tomllib
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parent.parent
LOCK = ROOT / "validation" / "refs.lock.toml"
FILE = "data/rockets/NDRT_2020/ndrt_2020_flight_data.csv"
# The file's SHA-256 at the pinned commit (RocketPy v1.13.0).
SHA256 = "5e64b446199988ac183b2f3965d76da27e57496bb2a33f78df73aa23fd18d7b8"
ATTEMPTS = 3


def pinned_commit() -> str:
    with LOCK.open("rb") as lock:
        pins = tomllib.load(lock)
    for git in pins["git"]:
        if git["name"] == "rocketpy":
            return git["commit"]
    raise SystemExit(f"{LOCK} pins no repository named rocketpy")


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__.strip().splitlines()[-1], file=sys.stderr)
        return 2
    out = pathlib.Path(sys.argv[1])
    url = f"https://raw.githubusercontent.com/RocketPy-Team/RocketPy/{pinned_commit()}/{FILE}"
    for attempt in range(1, ATTEMPTS + 1):
        try:
            with urllib.request.urlopen(url, timeout=60) as response:
                data = response.read()
            break
        except OSError as error:
            if attempt == ATTEMPTS:
                raise SystemExit(f"{url}: {error}") from error
            time.sleep(5 * attempt)
    digest = hashlib.sha256(data).hexdigest()
    if digest != SHA256:
        raise SystemExit(f"{url}: SHA-256 {digest}, expected {SHA256}")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_bytes(data)
    print(f"{out}: {len(data)} bytes, SHA-256 {digest}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
