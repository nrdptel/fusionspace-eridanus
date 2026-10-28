"""Take a fresh snapshot of ThrustCurve.org's solid motor catalog and write hpr-motor's copy of it
(M5.7b1, ADR-301).

ThrustCurve.org's API (https://www.thrustcurve.org/info/api.html) answers two requests this script
makes:

1. one POST to /api/v1/search.json with {"maxResults": 2000}, which returns every motor's record,
   in production or not;
2. POSTs to /api/v1/download.json with {"motorIds": [...], "data": "file"}, at most 250 motors a
   request, for every motor that isn't a hybrid and lists at least one data file. Each answer
   holds each file's bytes (base64) and its id, format, source and license.

The answers are kept verbatim under refs/snapshots/thrustcurve/<date>/ (gitignored, never
committed), with the requests and the time they were made. From them the script writes, under
crates/hpr-motor/data/thrustcurve/:

- records.json: every solid motor's record (type SU or reload), copied unchanged from the search
  answer, in motor id order, with the request and the snapshot's time. ThrustCurve.org grants no license for its records and reserves
  copyright; ADR-287 lets the project use them as published facts, with credit.
- files.json: every solid motor's data file as the download answer lists it, each entry copied
  unchanged but for its bytes ("data"), which go to the curve file, in motor id then file id
  order, with the snapshot's date.
- curves/<simfileId>.eng (RASP) or .rse (RockSim): each file's bytes, decoded from base64 and
  written byte for byte, but for a file whose license is "other", which its uploader restricted
  (ADR-288 section 2), or one held back (HELD, ADR-301 section 5): it is listed in files.json and
  no file is written. Curve files the snapshot doesn't list are removed.

It reads nothing else and decides nothing about the files: `cargo xtask motor-catalog` reads what
it writes, checks each file, and lists each file it leaves out with the reason. With --from DIR it
writes from an earlier snapshot's cached answers instead of asking the API, so the output can be
reproduced byte for byte.

Run from anywhere (standard library only):

    python3 validation/oracles/thrustcurve/snapshot.py
    python3 validation/oracles/thrustcurve/snapshot.py --from refs/snapshots/thrustcurve/2026-10-28
"""

import argparse
import base64
import datetime
import glob
import json
import os
import sys
import time
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
API = "https://www.thrustcurve.org/api/v1/"
AGENT = "fusionspace-hpr-snapshot/1 (+https://hpr.fusionspace.co)"
DATA = os.path.join(ROOT, "crates", "hpr-motor", "data", "thrustcurve")
SNAPSHOTS = os.path.join(ROOT, "refs", "snapshots", "thrustcurve")
SOLID = ("SU", "reload")
BATCH = 250
EXTENSION = {"RASP": ".eng", "RockSim": ".rse"}
# Files whose bytes this repository doesn't hold until the maintainer clears them (ADR-301
# section 5): listed in files.json, not written. `cargo xtask motor-catalog` leaves each out.
HELD = {"5f4294d20002e900000007de"}


def post(endpoint, body):
    request = urllib.request.Request(
        API + endpoint,
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json", "User-Agent": AGENT},
        method="POST",
    )
    with urllib.request.urlopen(request, timeout=300) as answer:
        if answer.status != 200:
            sys.exit(f"{endpoint}: HTTP {answer.status}")
        return answer.read()


def take(directory):
    """Asks the API, keeping each request and answer verbatim in `directory`."""
    os.makedirs(directory, exist_ok=True)
    captured = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    search = {"maxResults": 2000}
    raw = post("search.json", search)
    with open(os.path.join(directory, "search-request.json"), "w") as f:
        json.dump(search, f, indent=1)
    with open(os.path.join(directory, "search.json"), "wb") as f:
        f.write(raw)
    records = json.loads(raw)["results"]
    ids = sorted(
        r["motorId"] for r in records if r.get("type") in SOLID and (r.get("dataFiles") or 0) > 0
    )
    for n, start in enumerate(range(0, len(ids), BATCH)):
        body = {"motorIds": ids[start : start + BATCH], "data": "file"}
        time.sleep(2)
        raw = post("download.json", body)
        with open(os.path.join(directory, f"download-request-{n:02}.json"), "w") as f:
            json.dump(body, f, indent=1)
        with open(os.path.join(directory, f"download-{n:02}.json"), "wb") as f:
            f.write(raw)
    with open(os.path.join(directory, "CAPTURED"), "w") as f:
        f.write(captured + "\n")


def write(directory):
    """Writes hpr-motor's records, file list and curve files from the cached answers."""
    with open(os.path.join(directory, "CAPTURED")) as f:
        captured = f.read().strip()
    with open(os.path.join(directory, "search.json"), "rb") as f:
        search = json.loads(f.read())
    records = sorted(
        (r for r in search["results"] if r.get("type") in SOLID), key=lambda r: r["motorId"]
    )
    solid = {r["motorId"] for r in records}
    files = []
    curves = {}
    for path in sorted(glob.glob(os.path.join(directory, "download-[0-9][0-9].json"))):
        with open(path, "rb") as f:
            answer = json.loads(f.read())
        for entry in answer["results"]:
            if entry["motorId"] not in solid:
                sys.exit(f"{path}: {entry['motorId']} isn't a solid motor in the search answer")
            data = entry.get("data")
            listed = {k: v for k, v in entry.items() if k != "data"}
            files.append(listed)
            if entry.get("license") == "other" or entry["simfileId"] in HELD:
                continue
            if data is None or entry.get("format") not in EXTENSION:
                # Listed with what the answer says; the index writer names the reason.
                continue
            curves[entry["simfileId"] + EXTENSION[entry["format"]]] = base64.b64decode(data)
    files.sort(key=lambda e: (e["motorId"], e["simfileId"]))
    if len({e["simfileId"] for e in files}) != len(files):
        sys.exit("a file id appears twice in the download answers")
    with open(os.path.join(directory, "search-request.json")) as f:
        request = json.load(f)
    listed = {"source_url": API + "search.json", "request": request, "captured": captured}
    with open(os.path.join(DATA, "records.json"), "w", encoding="utf-8") as f:
        json.dump({**listed, "records": records}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    with open(os.path.join(DATA, "files.json"), "w", encoding="utf-8") as f:
        json.dump({"captured": captured, "files": files}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    out = os.path.join(DATA, "curves")
    os.makedirs(out, exist_ok=True)
    for name in os.listdir(out):
        if name not in curves:
            os.remove(os.path.join(out, name))
    for name, data in curves.items():
        with open(os.path.join(out, name), "wb") as f:
            f.write(data)
    motors_with_files = len({e["motorId"] for e in files})
    print(
        f"snapshot {captured}: {len(records)} solid motors "
        f"({sum(r['type'] == 'SU' for r in records)} single-use, "
        f"{sum(r['type'] == 'reload' for r in records)} reload), "
        f"{motors_with_files} with a data file, {len(files)} files, "
        f"{sum(e.get('license') == 'other' for e in files)} restricted, "
        f"{len(curves)} written"
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--from", dest="source", help="write from this snapshot's cached answers")
    args = parser.parse_args()
    if args.source:
        directory = os.path.abspath(args.source)
    else:
        directory = os.path.join(SNAPSHOTS, datetime.date.today().isoformat())
        take(directory)
    write(directory)


if __name__ == "__main__":
    main()
