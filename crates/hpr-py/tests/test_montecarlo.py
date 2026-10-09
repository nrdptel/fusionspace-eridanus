"""Monte Carlo from Python: `hpr.MonteCarlo` returns `hpr mc`'s runs as NumPy arrays, its failed
flights counted, equal bit for bit to `hpr mc --export`'s CSV from the same build and seed (M4.6b).

The command line is a release build, as the wheel is: `scripts/python-tests.sh` builds both, and
a debug build's last digits can differ from a release build's.
"""

import csv
import json
import math
import pathlib
import struct
import subprocess
import threading

import numpy
import pytest

from fusionspace import hpr

REPO = pathlib.Path(__file__).resolve().parents[3]
DESIGN = "validation/fixtures/ork/guides/level-1.ork"
RUNS = 40
SEED = 11
# Spaceport America, as the guide's Calisto flies from.
SITE = (32.990254, -106.974998, 1400.0)
RAIL = {"rail_length_m": 1.5, "inclination_deg": 85.0, "heading_deg": 30.0}
WIND = {"wind_speed_m_s": 4.0, "wind_from_deg": 250.0}
# Each of the eleven dispersions, by `MonteCarlo`'s argument and `hpr mc`'s option. A mass
# scattered by 50% leaves some flights a negative mass, so the run has failed flights to count.
DISPERSION = {
    "mass_sd_fraction": ("--mass-sd", 0.5),
    "cg_sd_m": ("--cg-sd", 0.01),
    "drag_sd_fraction": ("--drag-sd", 0.05),
    "impulse_sd_fraction": ("--impulse-sd", 0.03),
    "burn_time_sd_fraction": ("--burn-time-sd", 0.03),
    "delay_sd_s": ("--delay-sd", 1.0),
    "wind_sd_fraction": ("--wind-sd", 0.2),
    "wind_from_sd_deg": ("--wind-from-sd", 20.0),
    "inclination_sd_deg": ("--inclination-sd", 2.0),
    "heading_sd_deg": ("--heading-sd", 5.0),
    "deployment_lag_sd_s": ("--deployment-lag-sd", 0.3),
}


def hpr_mc(cli, *options):
    """`hpr mc`, the command line `cli`, on the design, from the site and rail above, with
    `options`; its JSON output."""
    latitude, longitude, elevation = SITE
    command = [
        str(cli),
        "mc",
        str(REPO / DESIGN),
        "--offline",
        "--json",
        f"--latitude={latitude!r}",
        f"--longitude={longitude!r}",
        f"--elevation={elevation!r}",
        f"--rail-length={RAIL['rail_length_m']!r}",
        f"--inclination={RAIL['inclination_deg']!r}",
        f"--heading={RAIL['heading_deg']!r}",
        f"--wind={WIND['wind_speed_m_s']!r}",
        f"--wind-from={WIND['wind_from_deg']!r}",
        *options,
    ]
    done = subprocess.run(command, capture_output=True, text=True, check=False)
    assert done.returncode == 0, done.stdout + done.stderr
    return json.loads(done.stdout)


def monte_carlo(repo, **options):
    rocket = hpr.Rocket.from_file(repo / DESIGN, recovery=True)
    site = hpr.Environment(*SITE, **WIND)
    return hpr.MonteCarlo(rocket, site, **RAIL, **options)


def scattered():
    return {name: value for name, (_, value) in DISPERSION.items()}


def scattered_options():
    return [f"{option}={value!r}" for option, value in DISPERSION.values()]


def bits(value):
    return struct.pack("<d", value)


@pytest.fixture(scope="module")
def runs(tmp_path_factory, hpr_cli):
    """The same run from Python and from `hpr mc --export`: Python's, the CSV's text, and the
    command's JSON output."""
    repo = REPO
    path = tmp_path_factory.mktemp("mc") / "run.csv"
    output = hpr_mc(
        hpr_cli, f"--runs={RUNS}", f"--seed={SEED}", *scattered_options(), f"--export={path}"
    )
    with open(path, newline="", encoding="utf-8") as file:
        text = file.read()
    return monte_carlo(repo, runs=RUNS, seed=SEED, **scattered()), text, output


def test_a_run_is_hpr_mcs_bit_for_bit(runs, csv_header):
    run, text, _ = runs
    header, *rows = list(csv.reader(text.splitlines()))
    # The header gives each column in words with its unit in brackets; the run keeps the names.
    assert [csv_header(name) for name in run.columns] == header
    assert "apogee [m]" in header and "max acceleration [m/s^2]" in header
    assert len(rows) == RUNS == run.runs
    # Every column the export has: the draw's lists by stage, motor and device among them.
    for name in ("motor_1_ejection_delay_offset_s", "device_1_deployment_lag_offset_s"):
        assert name in run
    for index, name in enumerate(run.columns):
        cells = [row[index] for row in rows]
        values = run[name]
        assert isinstance(values, numpy.ndarray) and values.shape == (RUNS,), name
        assert not values.flags.writeable, name
        if values.dtype.kind == "f":
            for value, cell in zip(values, cells):
                if cell == "":
                    assert math.isnan(value), name
                else:
                    assert bits(value) == bits(float(cell)), (name, value, cell)
        elif values.dtype.kind == "u":
            assert [str(value) for value in values] == cells, name
        else:
            assert values.dtype.kind == "U", name
            assert values.tolist() == cells, name
    # And the text itself, byte for byte.
    assert run.to_csv() == text


def test_the_failed_flights_are_counted(runs):
    run, _, output = runs
    outcome = run["outcome"]
    assert 0 < run.failed == int((outcome == "failed").sum()) < RUNS
    assert run.flown == RUNS - run.failed == output["flown"]
    assert run.failed == output["failed"]["count"]
    # Grouped as `hpr mc` groups them.
    assert run.failures == output["failed"]["reasons"]
    assert sum(failure["count"] for failure in run.failures) == run.failed
    failed = outcome == "failed"
    # A failed flight keeps its row, its draw and its reason, and has no figures.
    assert numpy.isnan(run["apogee_m"][failed]).all()
    assert not numpy.isnan(run["stage_1_dry_mass_scale"][failed]).any()
    assert (run["reason"][failed] != "").all()
    assert (run["failed_at"][~failed] == "").all()
    assert not numpy.isnan(run["apogee_m"][~failed]).any()


def test_the_nominal_flight_and_the_dispersion(runs):
    run, _, output = runs
    assert run.nominal["apogee"]["height_above_ground_m"] == output["nominal"]["apogee_m"]
    assert run.dispersion == output["dispersion"]
    assert run.seed == SEED == output["seed"]
    assert repr(run) == f"MonteCarlo(runs={RUNS}, seed={SEED}, failed={run.failed})"


def test_with_nothing_scattered_every_flight_is_the_nominal(repo):
    run = monte_carlo(repo, runs=3)
    assert run.failed == 0
    apogee = run.nominal["apogee"]["height_above_ground_m"]
    assert [bits(value) for value in run["apogee_m"]] == [bits(apogee)] * 3
    assert list(run) == run.keys() == run.columns and len(run) == len(run.columns)


def test_the_same_seed_flies_the_same_run_and_another_seed_another(repo):
    one = monte_carlo(repo, runs=4, seed=3, impulse_sd_fraction=0.03)
    assert one.to_csv() == monte_carlo(repo, runs=4, seed=3, impulse_sd_fraction=0.03).to_csv()
    other = monte_carlo(repo, runs=4, seed=4, impulse_sd_fraction=0.03)
    assert not numpy.array_equal(one["apogee_m"], other["apogee_m"])


@pytest.mark.parametrize("runs", [0, 100_001])
def test_a_run_of_no_flights_or_too_many_is_refused(repo, runs):
    with pytest.raises(hpr.HprError, match=f"runs={runs}: a run flies 1 to 100000 flights"):
        monte_carlo(repo, runs=runs)


@pytest.mark.parametrize("value", [-0.01, math.nan, math.inf])
def test_a_standard_deviation_out_of_its_domain_is_refused_by_name(repo, value):
    with pytest.raises(hpr.HprError, match=r"^cg_sd_m=.*: a standard deviation is a finite"):
        monte_carlo(repo, runs=2, cg_sd_m=value)


def test_a_column_that_is_not_there(repo):
    with pytest.raises(KeyError, match="no column `apogee`"):
        monte_carlo(repo, runs=1)["apogee"]


def test_a_wind_functions_exception_is_raised(repo):
    # Raised only off the calling thread: the nominal flight flies here, the run's flights on the
    # run's threads, so this is an exception from a flight of the run.
    caller = threading.get_ident()
    raised = []

    def wind(height_m):
        if threading.get_ident() != caller:
            raised.append(height_m)
            raise RuntimeError("no wind data that high")
        return (3.0, 0.0)

    rocket = hpr.Rocket.from_file(repo / DESIGN, recovery=True)
    site = hpr.Environment(*SITE, wind=wind)
    with pytest.raises(RuntimeError, match="no wind data that high"):
        hpr.MonteCarlo(rocket, site, **RAIL, runs=20, wind_sd_fraction=0.1)
    assert raised


def test_a_drag_function_flies_in_every_flight(repo):
    rocket = hpr.Rocket.from_file(repo / DESIGN, recovery=True)
    site = hpr.Environment(*SITE)

    def drag(mach, thrusting):
        return 0.5

    def flown(**options):
        return hpr.MonteCarlo(rocket, site, **RAIL, runs=3, **options)

    own, by_function = flown(), flown(drag=drag)
    assert by_function.failed == 0
    assert by_function["apogee_m"][0] != own["apogee_m"][0]
    # The function's drag, scattered as hpr's is.
    scaled = flown(drag=drag, drag_sd_fraction=0.1)
    assert len(set(scaled["apogee_m"].tolist())) == 3
