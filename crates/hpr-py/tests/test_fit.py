"""`FlightLog.fit`: a design's simulation fitted to its flight's log by least squares, the same
fit as `hpr fit` prints: each parameter with its 90% interval, each reading with where it came
from, the warnings of the flight it meets and the caveat on the interval that fails its test
(M7.3f)."""

import json
import math
import os
import pathlib
import subprocess

import pytest

from fusionspace import hpr

LEVEL_1 = "validation/fixtures/ork/guides/level-1.ork"
LEVEL_1_LOG = "validation/fixtures/logs/compare/level-1-flown.csv"
LEVEL_1_MAPPING = "validation/fixtures/logs/compare/level-1.mapping.json"
NDRT = "validation/designs/rocketpy-ndrt-2020-nose-to-tail.json"
SYNTHETIC = "validation/designs/synthetic-54mm-three-fin.json"
NDRT_LOG = "refs/rocketpy/data/rockets/NDRT_2020/ndrt_2020_flight_data.csv"

SITE = (41.7, -86.2, 230.0)
RAIL_M = 3.6


def ndrt_log(repo):
    """NDRT 2020's log: the file `HPR_NDRT_2020_LOG` names, which must then be a file, under
    the name RocketPy gives it; else RocketPy's checkout under `refs/`, or a skip where it
    isn't fetched. CI's Linux jobs fetch it with `scripts/fetch-rocketpy-logs.py` and name it
    in the variable, so on one of GitHub's Linux runners a run without the variable fails
    (issue #1648)."""
    named = os.environ.get("HPR_NDRT_2020_LOG")
    if named is not None:
        assert pathlib.Path(named).is_file(), f"HPR_NDRT_2020_LOG names {named}, which isn't a file"
        return pathlib.Path(named)
    on_a_linux_runner = "GITHUB_ACTIONS" in os.environ and os.environ.get("RUNNER_OS") == "Linux"
    assert not on_a_linux_runner, (
        "HPR_NDRT_2020_LOG isn't set on a Linux runner: CI's Linux jobs fetch NDRT 2020's log "
        "with scripts/fetch-rocketpy-logs.py and name it there"
    )
    if not (repo / NDRT_LOG).is_file():
        pytest.skip(f"{NDRT_LOG} isn't fetched")
    return repo / NDRT_LOG


def hpr_fit(cli, log, design, *options):
    """`hpr fit --json` of `log` by `design`, flown from `SITE` off a `RAIL_M` rail."""
    latitude, longitude, elevation = SITE
    command = [
        str(cli),
        "fit",
        str(log),
        str(design),
        "--offline",
        "--json",
        f"--latitude={latitude!r}",
        f"--longitude={longitude!r}",
        f"--elevation={elevation!r}",
        f"--rail-length={RAIL_M!r}",
        *options,
    ]
    done = subprocess.run(command, capture_output=True, text=True, check=False)
    assert done.returncode == 0, done.stdout + done.stderr
    return json.loads(done.stdout)


def same_fit(fitted, printed):
    """Whether the package's fit is the one the command printed, number for number."""
    assert len(fitted["parameters"]) == len(printed["parameters"]) == 6
    for mine, theirs in zip(fitted["parameters"], printed["parameters"]):
        name = mine["parameter"]
        assert name == theirs["parameter"]
        assert mine["value"] == theirs["value"], name
        assert mine["standard_error"] == theirs["standard_error"], name
        assert mine["stated"] == {"mean": theirs["stated_mean"], "sd": theirs["stated_sd"]}, name
        assert mine["interval_90"] == theirs["interval_90"], name
        assert mine["no_interval"] == theirs["no_interval"], name
        assert mine["caveat"] == theirs["caveat"], name
        assert mine["tested_range"] == theirs["tested_range"], name
        assert mine["outside_tested_range"] == theirs["outside_tested_range"], name
    for number in [
        "rms_m",
        "misfit_share",
        "converged",
        "clock_offset_s",
        "height_offset_m",
        "flights_flown",
        "stated_apogee_m",
        "fitted_apogee_m",
        "fitted_apogee_time_s",
        "fitted_max_mach",
    ]:
        assert fitted[number] == printed[number], number
    assert (fitted["wind_past_scan"] is None) == (printed["wind_past_scan"] is None)
    assert fitted["height_read_as"] == printed["height_read_as"]
    for reading in ["liftoff", "apogee", "height_track", "horizontal_track"]:
        mine, theirs = fitted["readings"][reading], printed["readings"][reading]
        assert (mine is None) == (theirs is None), reading
        if mine is not None:
            assert mine["source"] == theirs["source"], reading
            assert mine["provenance"] == theirs["provenance"], reading
    assert [w["issue"] for w in fitted["issue_warnings"]] == [w["issue"] for w in printed["issues"]]
    assert [w["message"] for w in fitted["issue_warnings"]] == [
        w["message"] for w in printed["issues"]
    ]
    return True


def by_name(fitted):
    """A fit's parameters, each under its name."""
    return {row["parameter"]: row for row in fitted["parameters"]}


def test_a_fit_is_the_one_hpr_fit_prints(repo, hpr_cli):
    # The guide's invented log, which the repository holds: flown in a wind and read 4% low.
    mapping = json.loads((repo / LEVEL_1_MAPPING).read_text())
    log = hpr.FlightLog.read(repo / LEVEL_1_LOG, mapping=mapping)
    rocket = hpr.Rocket.from_file(repo / LEVEL_1, recovery=True)
    fitted = log.fit(rocket, hpr.Environment(*SITE), RAIL_M)
    printed = hpr_fit(
        hpr_cli, repo / LEVEL_1_LOG, repo / LEVEL_1, f"--mapping={repo / LEVEL_1_MAPPING}"
    )
    assert same_fit(fitted, printed)

    # Each parameter, in the fit's order, with its 90% interval or why it has none.
    rows = by_name(fitted)
    assert list(rows) == [
        "drag_scale",
        "impulse_scale",
        "burn_time_scale",
        "wind_east_m_s",
        "wind_north_m_s",
        "mass_scale",
    ]
    for name in ["drag_scale", "impulse_scale", "mass_scale"]:
        row = rows[name]
        assert row["no_interval"] is None
        assert row["interval_90"]["low"] < row["value"] < row["interval_90"]["high"]
    assert (rows["burn_time_scale"]["interval_90"], rows["burn_time_scale"]["no_interval"]) == (
        None,
        "not_asked",
    )
    for wind in ["wind_east_m_s", "wind_north_m_s"]:
        assert (rows[wind]["interval_90"], rows[wind]["no_interval"]) == (None, "height_alone")
    # The site's air is calm. A log of height alone is told the wind its intervals were sized
    # in, 20 mph over √3, toward the east, and the text says so.
    assert rows["wind_east_m_s"]["stated"]["mean"] == pytest.approx(20 * 0.44704 / 3**0.5, abs=1e-12)
    assert rows["wind_north_m_s"]["stated"]["mean"] == 0.0
    assert (
        "\n  So the fit is told a wind of at least 5.16 m/s, the wind its intervals were sized in.\n"
        in fitted["text"]
    )
    assert " bins of 0.25 s from 1 s before liftoff, read as " in fitted["text"]
    # Each value lies inside the range the intervals were tested over: none is marked.
    assert rows["drag_scale"]["tested_range"] == {"low": 0.8, "high": 1.2}
    assert rows["impulse_scale"]["tested_range"] == {"low": 0.94, "high": 1.06}
    assert not any(row["outside_tested_range"] for row in rows.values())
    assert "†" not in fitted["text"]
    # A wind stronger than that, given, is the wind told: 7 m/s from the west blows east.
    windy = hpr.Environment(*SITE, wind_speed_m_s=7.0, wind_from_deg=270.0)
    told = by_name(log.fit(rocket, windy, RAIL_M))["wind_east_m_s"]["stated"]["mean"]
    assert told == pytest.approx(7.0, abs=1e-9)
    # Read 4% low: less impulse than the motor's file states, outside what its interval holds.
    assert rows["impulse_scale"]["interval_90"]["high"] < 1.0

    # Each reading, with where it came from.
    readings = fitted["readings"]
    for reading in ["liftoff", "apogee", "height_track"]:
        assert readings[reading]["source"] == "barometer"
        assert readings[reading]["provenance"] == "measured"
    assert readings["horizontal_track"] is None
    # A log of height alone has no track for a second answer in the wind to fit.
    assert fitted["wind_past_scan"] is None
    assert 0 < readings["height_track"]["bins_read"] <= readings["height_track"]["bins"]


def test_the_interval_that_fails_its_test_carries_the_caveat(repo):
    mapping = json.loads((repo / LEVEL_1_MAPPING).read_text())
    log = hpr.FlightLog.read(repo / LEVEL_1_LOG, mapping=mapping)
    fitted = log.fit(hpr.Rocket.from_file(repo / LEVEL_1), hpr.Environment(*SITE), RAIL_M)
    rows = by_name(fitted)
    # ADR-368: the drag scale's interval left 79 of 1,000 truths below it, where 50 are meant.
    caveat = rows["drag_scale"]["caveat"]
    assert (caveat["below"], caveat["flights"], caveat["meant"]) == (79, 1000, 50)
    # The committed report of the intervals' test holds the same count.
    report = json.loads((repo / "validation/reports/fit-intervals.json").read_text())
    (set_b,) = [row for row in report["sets"] if row["set"] == "b"]
    (check,) = [row for row in set_b["checks"] if row["asked"] == "drag-scale"]
    counts = check["test"]["counts"]
    assert (counts["below"], counts["given"]) == (caveat["below"], caveat["flights"])
    # Which way it errs: the fitted drag high, a flight predicted with it low.
    assert (
        "79 truths lay below the interval, where a 90% interval leaves 50. So its lower end can "
        "read too high: take the fitted drag scale as reading high, and a flight predicted with "
        "it as reading low." in caveat["message"]
    )
    assert caveat["message"].endswith("https://github.com/nrdptel/fusionspace-eridanus/issues/1108")
    for name in ["impulse_scale", "burn_time_scale", "wind_east_m_s", "wind_north_m_s", "mass_scale"]:
        assert rows[name]["caveat"] is None
    # The text prints the caveat with the intervals.
    marked = [line for line in fitted["text"].splitlines() if " *  " in line]
    assert len(marked) == 1 and marked[0].startswith("  drag scale  ")
    assert "\n  * This interval fails its test: the caveat below.\n" in fitted["text"]
    assert f"Caveat. {caveat['message']}" in fitted["text"]


def test_a_fit_carries_the_warnings_its_flight_meets(repo):
    mapping = json.loads((repo / LEVEL_1_MAPPING).read_text())
    log = hpr.FlightLog.read(repo / LEVEL_1_LOG, mapping=mapping)
    rocket = hpr.Rocket.from_file(repo / LEVEL_1, recovery=True)
    site = hpr.Environment(*SITE)
    fitted = log.fit(rocket, site, RAIL_M)
    warnings = fitted["issue_warnings"]
    # The level 1 rocket's margin always carries issue #172's warning.
    assert 172 in [w["issue"] for w in warnings]
    for warning in warnings:
        assert warning["message"].startswith(f"issue #{warning['issue']}: ")
        assert warning["url"].endswith(f"/issues/{warning['issue']}")
        assert f"Warning. {warning['message']}" in fitted["text"]
    # They are the warnings of the flight at the fitted parameters: the stated flight, flown
    # higher and so faster, meets at least these.
    stated = hpr.Flight(rocket, site, RAIL_M)
    assert {w["issue"] for w in warnings} <= {w["issue"] for w in stated.issue_warnings}


def test_a_fit_refuses_what_it_cant_fit(repo, tmp_path):
    rocket = hpr.Rocket.from_file(repo / LEVEL_1)
    site = hpr.Environment(*SITE)
    mapping = json.loads((repo / LEVEL_1_MAPPING).read_text())
    log = hpr.FlightLog.read(repo / LEVEL_1_LOG, mapping=mapping)
    with pytest.raises(hpr.HprError, match='height_reads is "feet"'):
        log.fit(rocket, site, RAIL_M, height_reads="feet")
    # A stated kind of height is the one fitted.
    assert log.fit(rocket, site, RAIL_M, height_reads="height_climbed")["height_read_as"] == (
        "height_climbed"
    )
    # A log that never leaves the pad has no apogee to fit to.
    flat = tmp_path / "flat.csv"
    header = (repo / LEVEL_1_LOG).read_text().splitlines()[0]
    columns = len(header.split(","))
    rows = [",".join([f"{step * 0.05}"] + ["0"] * (columns - 1)) for step in range(400)]
    flat.write_text("\n".join([header, *rows]) + "\n")
    with pytest.raises(hpr.HprError, match="its apogee is withheld"):
        hpr.FlightLog.read(flat, mapping=mapping).fit(rocket, site, RAIL_M)


def test_a_public_flight_fits_as_hpr_fit_prints_it(repo, hpr_cli):
    # NDRT 2020's Featherweight Raven log, by RocketPy's example of the rocket (`ndrt_log`).
    path = ndrt_log(repo)
    log = hpr.FlightLog.read(path)
    rocket = hpr.Rocket.from_file(repo / NDRT, "example", recovery=True)
    fitted = log.fit(rocket, hpr.Environment(*SITE), RAIL_M)
    printed = hpr_fit(hpr_cli, path, repo / NDRT, "--config=example")
    assert same_fit(fitted, printed)
    rows = by_name(fitted)
    # The flight points to less drag than the design states, as the committed report of the
    # public flights' fits holds for it (`from-boost.json`: 0.71).
    drag = rows["drag_scale"]
    assert abs(drag["value"] - 0.71) < 0.1
    assert drag["interval_90"]["high"] < 1.0
    assert drag["caveat"]["below"] == 79
    # Below the 0.8 the intervals were tested down to, and marked so.
    assert drag["outside_tested_range"] and drag["value"] < drag["tested_range"]["low"]
    assert not rows["mass_scale"]["outside_tested_range"]
    assert (
        "  † The drag scale lies outside 0.800 to 1.200, the range the intervals were tested over."
        in fitted["text"]
    )
    assert fitted["readings"]["apogee"]["source"] == "barometer"
    assert fitted["readings"]["apogee"]["provenance"] == "measured"
    assert [w["issue"] for w in fitted["issue_warnings"]] == [73, 18, 172]
    assert fitted["misfit_share"] < 0.03
    print(fitted["text"])


def test_a_fit_gives_the_second_answer_it_leaves_past_the_wind_scan(repo, hpr_cli, tmp_path):
    # The 54 mm design flown in 13 m/s toward the east, past the 12 m/s the fit's scan of the wind
    # reaches, and fitted from calm air (issue #1646, ADR-382 §7). `hpr sim` flies it; its heights
    # and its track, as latitude and longitude about the site, are the log.
    design = repo / SYNTHETIC
    latitude, longitude, elevation = SITE
    export = tmp_path / "flown.csv"
    flown = subprocess.run(
        [
            str(hpr_cli),
            "sim",
            str(design),
            "--offline",
            "--wind=13",
            "--wind-from=270",
            f"--export={export}",
            "--interval=0.05",
            f"--latitude={latitude!r}",
            f"--longitude={longitude!r}",
            f"--elevation={elevation!r}",
            f"--rail-length={RAIL_M!r}",
        ],
        capture_output=True,
        text=True,
        check=False,
    )
    assert flown.returncode == 0, flown.stdout + flown.stderr
    header, *lines = export.read_text().splitlines()
    names = header.split(",")
    columns = [
        names.index(name)
        for name in [
            "time [s]",
            "height above ground [m]",
            "position east [m]",
            "position north [m]",
        ]
    ]
    rows = [[float(line.split(",")[column]) for column in columns] for line in lines]
    first, last = rows[0], rows[-1]
    # The mean radius of the Earth (IUGG R1), as the fit turns a fix's angles into meters.
    radius_m = 6_371_008.8

    def fix(time_s, row):
        north_deg = math.degrees((row[3] - first[3]) / radius_m)
        east_deg = math.degrees(
            (row[2] - first[2]) / (radius_m * math.cos(math.radians(latitude)))
        )
        return f"{time_s!r},{row[1]!r},{latitude + north_deg!r},{longitude + east_deg!r}"

    # Ten seconds on the pad, the flight, ten seconds where it ended.
    pad_s = 10.0
    track = ["Time,Altitude,Lat,Lon"]
    track += [fix(step * 0.05, first) for step in range(200)]
    track += [fix(row[0] + pad_s, row) for row in rows]
    track += [fix(last[0] + pad_s + step * 0.05, last) for step in range(1, 201)]
    log_path = tmp_path / "track.csv"
    log_path.write_text("\n".join(track) + "\n")
    mapping = {
        "header_line": 1,
        "separator": ",",
        "decimal_comma": False,
        "clock": 0,
        "columns": [
            {"name": "Time", "quantity": None, "unit": "s"},
            {"name": "Altitude", "quantity": "altitude_above_ground", "unit": "m"},
            {"name": "Lat", "quantity": "latitude", "unit": "deg"},
            {"name": "Lon", "quantity": "longitude", "unit": "deg"},
        ],
    }
    mapping_path = tmp_path / "track.mapping.json"
    mapping_path.write_text(json.dumps(mapping))

    log = hpr.FlightLog.read(log_path, mapping=mapping)
    fitted = log.fit(hpr.Rocket.from_file(design), hpr.Environment(*SITE), RAIL_M)
    printed = hpr_fit(hpr_cli, log_path, design, f"--mapping={mapping_path}")
    assert same_fit(fitted, printed)
    # Which way it errs: the fitted wind blows the other way, and its interval misses the flight's.
    east = by_name(fitted)["wind_east_m_s"]
    assert east["value"] < 0 and east["interval_90"]["high"] < 13.0
    # So the fit gives the answer it left, which lies by the flight's wind and fits far better.
    past, theirs = fitted["wind_past_scan"], printed["wind_past_scan"]
    assert set(past) == set(theirs) == {
        "wind_east_m_s",
        "wind_north_m_s",
        "speed_m_s",
        "from_deg",
        "chi2",
        "standing_chi2",
        "message",
    }
    for number in set(past) - {"message"}:
        assert past[number] == theirs[number], number
    assert abs(past["wind_east_m_s"] - 13.0) < 1.0 and abs(past["wind_north_m_s"]) < 0.5
    assert past["speed_m_s"] == math.hypot(past["wind_east_m_s"], past["wind_north_m_s"]) > 12.0
    assert abs(past["from_deg"] - 270.0) < 1.0
    assert past["chi2"] < 0.25 * past["standing_chi2"]
    assert past["message"].startswith("The wind has a second answer, which the fit didn't take.")
    assert f"\n\nCaveat. {past['message']}\n" in fitted["text"] + "\n"
    # The command adds how to fit again from that wind on its own line of options.
    said = math.ceil(past["speed_m_s"] * 10 - 1e-6) / 10
    assert theirs["message"] == f"{past['message']} On the command line: `--wind {said:.1f} --wind-from 270`."
