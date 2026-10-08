"""The builder's example rocket, built and flown from Python.

`crates/hpr/examples/build_and_fly.rs` builds this rocket with the Rust builder, and CI keeps
what it prints in `build_and_fly.output.txt`. These tests build the same rocket from Python and
print the same lines with the same formats: the bindings add no physics, so every digit agrees.
"""

import json

import numpy as np
import pytest

from fusionspace import hpr


def example_rocket(fins: bool = True) -> hpr.Rocket:
    """The Rust example's rocket, part by part, with the same numbers; with no fins if `fins` is
    false."""
    motor = hpr.Motor.from_catalog("H54", delay_s=10.0)
    rocket = hpr.Rocket("My 54 mm rocket", 0.0563)
    rocket.add_nose(
        "ogive",
        0.22,
        "abs",
        wall_m=0.0015,
        shoulder_length_m=0.06,
        shoulder_wall_m=0.0015,
        capped_shoulder=True,
    )
    rocket.add_tube(0.9, 0.00115, "kraft_phenolic")
    rocket.add_motor_tube(0.2, 0.029, 0.001, "kraft_phenolic", overhang_m=0.005)
    if fins:
        rocket.add_fins(
            3,
            root_chord_m=0.1,
            tip_chord_m=0.04,
            span_m=0.045,
            sweep_m=0.05,
            thickness_m=0.003175,
            material="birch_plywood",
            cross_section="rounded",
        )
    rocket.add_mass(
        0.2,
        position="top",
        offset_m=0.07,
        packed_length_m=0.15,
        packed_diameter_m=0.05,
        name="recovery bay",
    )
    rocket.set_motor(motor)
    rocket.add_parachute("parachute", diameter_m=0.9, trigger="motor_delay", motor=0)
    return rocket


def example_flight(rocket: hpr.Rocket) -> hpr.Flight:
    environment = hpr.Environment(
        32.99, -106.97, 1400.0, wind_speed_m_s=5.0, wind_from_deg=270.0
    )
    return hpr.Flight(rocket, environment, 1.8, inclination_deg=85.0, heading_deg=270.0)


def printed(rocket: hpr.Rocket, flight: hpr.Flight) -> str:
    """The Rust example's output, line for line, from the Python objects."""
    motor = hpr.Motor.from_catalog("H54")
    end_s = motor.thrust_curve()[0][-1]
    liftoff, spent = rocket.margin(0.0, 0.3), rocket.margin(end_s, 0.3)
    full, empty = rocket.mass_properties(0.0), rocket.mass_properties(end_s)
    apogee = next(e for e in flight.events if e["kind"] == "apogee")
    landing = flight.landing
    lines = [
        f"{rocket.name} on a {rocket.configuration}",
        "Not yet validated: see the Accuracy page before trusting these numbers.",
        "",
        "                                 liftoff     spent",
        f"mass (kg)                        {full['mass_kg']:>7.3f} {empty['mass_kg']:>9.3f}",
        "center of gravity (m from nose)  "
        f"{-full['cg_m'][2]:>7.3f} {-empty['cg_m'][2]:>9.3f}",
        "center of pressure (m from nose) "
        f"{liftoff['cp_station_m']:>7.3f} {spent['cp_station_m']:>9.3f}",
        "stability margin (calibres)      "
        f"{liftoff['margin_cal']:>7.2f} {spent['margin_cal']:>9.2f}",
        "",
        "From a 1.8 m rail leaning 5° west, in 5 m/s of wind from the west:",
        f"Rail exit:  {flight.rail_exit_speed_m_s:.1f} m/s",
        f"Apogee:     {flight.apogee_m:.1f} m above the pad, at {flight.apogee_time_s:.2f} s, "
        f"{-apogee['sample']['cg_enu_m'][0]:.0f} m west of it",
        f"Top speed:  {flight.max_speed_m_s:.0f} m/s (Mach {flight.max_mach:.2f})",
        f"Landing:    {landing['east_m']:.0f} m east of the pad, at "
        f"{landing['descent_rate_m_s']:.1f} m/s, at {landing['time_s']:.1f} s",
    ]
    return "\n".join(lines) + "\n"


def test_prints_what_the_rust_example_prints(repo):
    rocket = example_rocket()
    expected = (repo / "crates/hpr/examples/build_and_fly.output.txt").read_text(
        encoding="utf-8"
    )
    assert printed(rocket, example_flight(rocket)) == expected


def test_a_flight_is_deterministic():
    rocket = example_rocket()
    first, second = example_flight(rocket), example_flight(rocket)
    assert first.to_json() == second.to_json()
    for name in first.columns:
        assert np.array_equal(first[name], second[name])


def test_the_recording_is_numpy_arrays():
    flight = example_flight(example_rocket())
    series = flight.series
    assert list(series) == flight.columns
    time_s = series["time_s"]
    assert isinstance(time_s, np.ndarray) and time_s.dtype == np.float64
    assert all(array.shape == time_s.shape for array in series.values())
    assert np.all(np.diff(time_s) > 0)
    # Its peak height is the apogee: the step ends bracket it, so the recorded peak is at most
    # the apogee, and within a meter of it.
    height_m = flight["height_above_ground_m"]
    assert flight.apogee_m - 1.0 < height_m.max() <= flight.apogee_m
    # The arrays are read-only copies of the flight's record.
    with pytest.raises(ValueError):
        height_m[0] = 0.0


def test_a_recording_at_an_interval():
    rocket = example_rocket()
    environment = hpr.Environment(32.99, -106.97, 1400.0)
    flight = hpr.Flight(rocket, environment, 1.8, interval_s=0.5)
    time_s = flight["time_s"]
    # A row at every multiple of 0.5 s within the flight, and one at every event.
    grid = np.arange(0.0, time_s[-1], 0.5)
    assert np.all(np.isin(grid, time_s))
    event_times = [event["time_s"] for event in flight.events]
    assert set(time_s) == set(grid) | set(event_times)


def test_events_come_in_order():
    flight = example_flight(example_rocket())
    kinds = [event["kind"] for event in flight.events]
    for first, then in [("liftoff", "rail_exit"), ("rail_exit", "burnout"), ("burnout", "apogee")]:
        assert kinds.index(first) < kinds.index(then)
    assert kinds[-1] == "ground_hit"
    times = [event["time_s"] for event in flight.events]
    assert times == sorted(times)
    deployment = next(e for e in flight.events if e["kind"] == "deployment")
    assert deployment["index"] == 0


def test_a_subsonic_flight_raises_no_envelope_flag():
    assert example_flight(example_rocket()).envelope_flags == []


def test_a_flight_past_the_validated_range_is_flagged(repo):
    """The synthetic two-stage rocket flies as one stack to about Mach 1.6, past the fastest
    public flight compared with an independent reference."""
    rocket = hpr.Rocket.from_file(
        repo / "validation/designs/synthetic-two-stage-75mm-54mm.json"
    )
    flight = hpr.Flight(rocket, hpr.Environment(32.99, -106.97, 1400.0), 1.5)
    [flag] = flight.envelope_flags
    assert flag["flag"] == "beyond_validated_range"
    assert flag["value"] == flight.max_mach
    assert flag["value"] == flight.summary["max_mach"]["value"]
    assert flag["time_s"] == flight.summary["max_mach"]["time_s"]
    assert f"reaches Mach {flight.max_mach:.2f}" in flag["message"]
    assert set(flag) == {"flag", "value", "time_s", "height_above_ground_m", "message"}


def test_a_rocket_without_fins_is_unstable_under_power():
    """With no fins the nose alone carries the normal force, ahead of the center of mass, so the
    static margin is below zero while the motor burns. With them, it is above zero."""
    flight = example_flight(example_rocket(fins=False))
    least = flight.summary["min_powered_static_margin_cal"]
    assert least["value"] < 0.0
    [flag] = [f for f in flight.envelope_flags if f["flag"] == "unstable_under_power"]
    assert flag["value"] == least["value"]
    assert flag["time_s"] == least["time_s"]
    assert "unstable under power" in flag["message"]
    assert "its apogee is not a prediction" in flag["message"]
    stable = example_flight(example_rocket()).summary["min_powered_static_margin_cal"]
    assert stable["value"] > 0.0



def test_a_rocket_unstable_where_it_has_no_margin_is_flagged(repo, tmp_path):
    """Valetudo with no fins and a conical tail narrowing from 40.45 mm to 22 mm radius: hpr can
    give no static margin under power, but the pitching moment turns the rocket away from its
    path, and that raises the second unstable flag."""
    design = json.loads((repo / "validation/designs/rocketpy-valetudo.json").read_text())
    components = design["stages"][0]["components"]
    children = components[1]["children"]
    components[1]["children"] = [c for c in children if c["id"] != "fins-1"]
    assert len(components[1]["children"]) == len(children) - 1
    components.append(
        {
            "id": "tail-1",
            "name": "",
            "part": {
                "transition": {
                    "shape": {"kind": "conical"},
                    "clipped": False,
                    "length_m": 0.08,
                    "fore_radius_m": 0.04045,
                    "aft_radius_m": 0.022,
                    "wall": {"kind": "shell", "thickness_m": 0.002},
                    "fore_shoulder": None,
                    "aft_shoulder": None,
                    "material": {
                        "name": "Fiberglass laminate (G10/FR-4)",
                        "density": {"kind": "bulk", "kg_m3": 1800.0},
                    },
                }
            },
        }
    )
    path = tmp_path / "narrow-tail.json"
    path.write_text(json.dumps(design))
    flight = hpr.Flight(hpr.Rocket.from_file(path), hpr.Environment(32.99, -106.97, 1400.0), 3.0)
    assert flight.summary["min_powered_static_margin_cal"] is None
    slope = flight.summary["max_powered_moment_slope_per_rad"]
    assert slope["value"] > 0.0
    [flag] = [f for f in flight.envelope_flags if f["flag"].startswith("unstable")]
    assert flag["flag"] == "unstable_without_margin"
    assert flag["value"] == slope["value"]
    assert flag["time_s"] == slope["time_s"]
    assert "its apogee is not a prediction" in flag["message"]

def test_a_subsonic_flight_meets_no_shape_drag_issue():
    # Only #18, which every flight on hpr's drag raises (ADR-190), and #172, which every flight
    # with a margin raises (ADR-181).
    warnings = example_flight(example_rocket()).issue_warnings
    assert [warning["issue"] for warning in warnings] == [18, 172]


def test_a_supersonic_ogive_nose_meets_its_drag_issues_by_number(repo):
    """The synthetic two-stage rocket, past Mach 1: its tangent-ogive nose meets #67 and #222,
    and its conical interstage, a shoulder, #67; base drag (#68) and friction (#18) need no
    part. Its rail buttons put its center of gravity off the axis (#219), and past the body's
    supersonic join's start its damping stations leave their centers of pressure (#106): two
    issues of the flight's path, whose signs depend on the case (ADR-201)."""
    rocket = hpr.Rocket.from_file(
        repo / "validation/designs/synthetic-two-stage-75mm-54mm.json"
    )
    flight = hpr.Flight(rocket, hpr.Environment(32.99, -106.97, 1400.0), 1.5)
    warnings = flight.issue_warnings
    assert [warning["issue"] for warning in warnings] == [67, 68, 222, 18, 172, 219, 106]
    assert [warning["parts"] for warning in warnings] == [
        ["nose", "interstage"],
        [],
        ["nose"],
        [],
        [],
        [],
        ["nose", "sustainer-airframe", "interstage"],
    ]
    for warning in warnings:
        assert warning["max_mach"] == flight.max_mach
        assert warning["url"].endswith(f"/issues/{warning['issue']}")
        assert f"issue #{warning['issue']}" in warning["message"]
        assert set(warning) == {
            "issue", "kind", "url", "max_mach", "time_s", "parts", "message"
        }
    assert [warning["kind"] for warning in warnings] == (
        ["drag"] * 4 + ["stability"] + ["flight"] * 2
    )
    # Flown on a drag table of its own, the flight's drag isn't hpr's, so it meets none of the
    # drag issues, #18 among them; the normal force is still hpr's, so #172 stays, and so do
    # the flight path's, which no drag changes.
    table = hpr.DragTable([(0.0, 0.5), (3.0, 0.5)])
    tabled = hpr.Flight(
        rocket, hpr.Environment(32.99, -106.97, 1400.0), 1.5, drag_table=table
    )
    assert tabled.max_mach > 1.0
    assert [warning["issue"] for warning in tabled.issue_warnings] == [172, 219, 106]
