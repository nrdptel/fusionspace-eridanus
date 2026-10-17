"""The invented GPX samples as gpxpy reads them, the outside reading `hpr_flightdata::gpx` is
checked against (M7.1d1).

gpxpy 1.6.2 (Apache-2.0) is run as the outside reader; none of its code is ported. It is pinned in
`validation/oracles/pyproject.toml` and lives in `refs/venv`. The samples are the invented files
`crates/hpr-flightdata/tests/samples/gpx-*.gpx`: made-up positions, heights and times, no real
flight. For each, in the file's order, this script writes gpxpy's reading to
`crates/hpr-flightdata/tests/fixtures/gpx/gpxpy.json`:

- the file's name and SHA-256, its `version` and `creator`;
- each track's name and segments, each segment's points;
- the waypoints, and each route's name and points.

A point is its latitude, longitude and elevation, its time as whole microseconds since
1970-01-01T00:00:00 UTC (gpxpy reads a time to the microsecond, and moves one with an offset to
UTC; one with no zone it leaves naive, and this script reads it as UTC, as the GPX schemas say
every time is), and, for track points, its satellite count, its three dilutions of precision and,
in GPX 1.0, its course and speed; `null` where gpxpy read none. Floats are written by Python's
`repr`, which round-trips a double exactly. The samples use only what both readers read alike: a
time to the microsecond or coarser (gpxpy cuts finer digits), numbers with no exponent, and no
value either reader refuses or drops.

Run from the repository root:

    refs/venv/bin/python -I validation/oracles/gpx/gpxpy_read.py
"""

import datetime
import hashlib
import json
import sys
from pathlib import Path

import gpxpy

VERSION = "1.6.2"
SAMPLES = Path("crates/hpr-flightdata/tests/samples")
OUT = Path("crates/hpr-flightdata/tests/fixtures/gpx/gpxpy.json")
EPOCH = datetime.datetime(1970, 1, 1, tzinfo=datetime.timezone.utc)
MICROSECOND = datetime.timedelta(microseconds=1)


def microseconds(time):
    """A gpxpy time as whole microseconds since the epoch, UTC; a naive one read as UTC."""
    if time is None:
        return None
    if time.tzinfo is None:
        time = time.replace(tzinfo=datetime.timezone.utc)
    return (time - EPOCH) // MICROSECOND


def point(p, track):
    """A point as gpxpy reads it."""
    read = {
        "lat": p.latitude,
        "lon": p.longitude,
        "ele": p.elevation,
        "time_us": microseconds(p.time),
        "name": p.name,
    }
    if track:
        read.update(
            sat=p.satellites,
            hdop=p.horizontal_dilution,
            vdop=p.vertical_dilution,
            pdop=p.position_dilution,
            speed=p.speed,
            course=p.course,
        )
    return read


def main():
    if gpxpy.__version__ != VERSION:
        sys.exit(f"gpxpy {gpxpy.__version__}, not {VERSION}: install refs/venv")
    files = []
    for path in sorted(SAMPLES.glob("gpx-*.gpx")):
        data = path.read_bytes()
        gpx = gpxpy.parse(data.decode("utf-8"))
        files.append(
            {
                "file": path.name,
                "sha256": hashlib.sha256(data).hexdigest(),
                "version": gpx.version,
                "creator": gpx.creator,
                "tracks": [
                    {
                        "name": track.name,
                        "segments": [
                            [point(p, True) for p in segment.points] for segment in track.segments
                        ],
                    }
                    for track in gpx.tracks
                ],
                "waypoints": [point(p, False) for p in gpx.waypoints],
                "routes": [
                    {"name": route.name, "points": [point(p, False) for p in route.points]}
                    for route in gpx.routes
                ],
            }
        )
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps({"gpxpy": VERSION, "files": files}, indent=1) + "\n")
    points = sum(
        len(segment) for f in files for track in f["tracks"] for segment in track["segments"]
    )
    print(f"{len(files)} files, {points} track points -> {OUT}")


if __name__ == "__main__":
    main()
