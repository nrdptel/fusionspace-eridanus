"""Motors and rockets read from files: the repository's public ones."""

import json
import re
import struct
import subprocess

import numpy as np
import pytest

from fusionspace import hpr

CURVES = "crates/hpr-motor/data/thrustcurve/curves"
CALISTO = "validation/designs/rocketpy-calisto-tests-motor-at-minus-1.373.json"
# A `.ork` whose one parachute opens at the motor's ejection charge.
LEVEL_1 = "validation/fixtures/ork/guides/level-1.ork"


def test_a_motor_from_an_eng_file(repo):
    path = repo / CURVES / "5f4294d20002e90000000724.eng"
    motor = hpr.Motor.from_file(path, delay_s=7.0)
    same = hpr.Motor.from_eng(path.read_text(encoding="utf-8"), delay_s=7.0)
    assert motor.designation == same.designation
    assert motor.delay_s == 7.0
    times_s, thrusts_n = motor.thrust_curve()
    assert times_s.shape == thrusts_n.shape
    assert np.all(np.diff(times_s) > 0)
    # The curve's impulse, by the trapezoidal rule from the curve's first point at zero thrust.
    trapezoid = getattr(np, "trapezoid", None) or np.trapz  # NumPy 2 renamed it
    impulse_ns = trapezoid(np.concatenate(([0.0], thrusts_n)), np.concatenate(([0.0], times_s)))
    assert motor.total_impulse_ns == pytest.approx(impulse_ns, rel=1e-12)


def test_a_motor_from_an_rse_file(repo):
    path = repo / CURVES / "5f923edb1bca5800041716ab.rse"
    motor = hpr.Motor.from_file(path)
    assert motor.designation == hpr.Motor.from_rse(path.read_text(encoding="utf-8")).designation
    assert motor.delay_s is None
    assert motor.propellant_mass_kg > 0.0


def test_a_motor_file_of_another_kind_is_refused(repo):
    with pytest.raises(hpr.HprError, match="a motor file is"):
        hpr.Motor.from_file(repo / CALISTO)


def test_a_rocket_from_its_json(repo):
    rocket = hpr.Rocket.from_file(repo / CALISTO)
    assert rocket.configuration == "example"
    assert rocket.name.startswith("RocketPy example")
    environment = hpr.Environment(32.990254, -106.974998, 1400.0)
    flight = hpr.Flight(rocket, environment, 5.2, inclination_deg=85.0)
    assert flight.apogee_m > 2000.0
    assert flight.landing is not None


def test_a_design_read_back_flies_the_same(repo, tmp_path):
    rocket = hpr.Rocket.from_file(repo / CALISTO)
    copy = tmp_path / "calisto.json"
    copy.write_text(rocket.design_json(), encoding="utf-8")
    environment = hpr.Environment(32.990254, -106.974998, 1400.0)
    first = hpr.Flight(rocket, environment, 5.2)
    second = hpr.Flight(hpr.Rocket.from_file(copy, "example"), environment, 5.2)
    assert first.to_json() == second.to_json()


def level_1_with(repo, tmp_path, event):
    """The level 1 design, its parachute deployed at `event`."""
    text = (repo / LEVEL_1).read_text(encoding="utf-8")
    assert text.count("<deployevent>ejection</deployevent>") == 1
    path = tmp_path / "level-1.ork"
    path.write_text(
        text.replace("<deployevent>ejection</deployevent>", f"<deployevent>{event}</deployevent>"),
        encoding="utf-8",
    )
    return path


def test_a_files_parachute_flies_with_recovery(repo):
    site = hpr.Environment(0.0, 0.0, 0.0)
    unslowed = hpr.Rocket.from_file(repo / LEVEL_1)
    assert any("recovery=True would fly them" in note for note in unslowed.notes)
    slowed = hpr.Rocket.from_file(repo / LEVEL_1, recovery=True)
    assert not any("recovery device" in note for note in slowed.notes)
    falls, descends = hpr.Flight(unslowed, site, 1.5), hpr.Flight(slowed, site, 1.5)
    # `hpr sim` lands it at 4.4 m/s under `Parachute`, opened at the charge, 12.14 s.
    assert descends.landing["descent_rate_m_s"] == pytest.approx(4.4, abs=0.05)
    assert falls.landing["descent_rate_m_s"] > 20.0
    (deployment,) = [event for event in descends.events if event["kind"] == "deployment"]
    assert deployment["time_s"] == pytest.approx(12.14, abs=0.005)
    # The file's parachute is parachute 0, so one added after it can be released by it.
    slowed.add_parachute("main", cd_s_m2=2.0, trigger="altitude", altitude_m=300.0, released_by=0)
    assert hpr.Flight(slowed, site, 1.5).landing is not None


def test_a_files_parachute_that_never_opens_is_named(repo, tmp_path):
    rocket = hpr.Rocket.from_file(level_1_with(repo, tmp_path, "never"), recovery=True)
    assert "`Parachute` never opens: the file sets it to never" in rocket.notes


def test_a_files_recovery_hpr_cant_fly_is_refused(repo, tmp_path):
    with pytest.raises(hpr.HprError, match="the file's recovery can't be flown"):
        hpr.Rocket.from_file(level_1_with(repo, tmp_path, "somewhen"), recovery=True)


# A main for the level 1 design, beside its parachute at the charge: opened half a second after
# it falls past 150 m, on OpenRocket's own drag coefficient, and never in the second configuration.
MAIN = """<parachute>
                <name>Main</name>
                <id>6a1d0000-0000-4000-8000-0000000000f1</id>
                <axialoffset method="top">0.25</axialoffset>
                <packedlength>0.08</packedlength>
                <packedradius>0.03</packedradius>
                <cd>auto</cd>
                <material type="surface" density="0.067">Ripstop nylon</material>
                <deployevent>altitude</deployevent>
                <deployaltitude>150.0</deployaltitude>
                <deploydelay>0.5</deploydelay>
                <deploymentconfiguration configid="6a1d0000-0000-4000-8000-0000000000c2">
                  <deployevent>never</deployevent>
                </deploymentconfiguration>
                <diameter>1.5</diameter>
                <linecount>8</linecount>
                <linelength>1.2</linelength>
                <linematerial type="line" density="0.0025">Tubular nylon</linematerial>
              </parachute>
              """
SECOND = "6a1d0000-0000-4000-8000-0000000000c2"


def level_1_with_a_main(repo, tmp_path):
    """The level 1 design with `MAIN` after its parachute."""
    text = (repo / LEVEL_1).read_text(encoding="utf-8")
    assert text.count("</parachute>") == 1
    path = tmp_path / "level-1-main.ork"
    path.write_text(text.replace("<masscomponent>", MAIN + "<masscomponent>", 1), encoding="utf-8")
    return path


def two_stage_with_chutes(repo, tmp_path):
    """The two-stage example with a parachute in each stage, the sustainer's at its charge and the
    booster's at apogee, and a booster that never separates: one stack, so `Rocket.from_file`
    flies it."""
    text = two_stage_ork(repo, tmp_path).read_text(encoding="utf-8")
    chute = (
        "<parachute><name>{name} chute</name><id>{name}-chute</id>"
        '<axialoffset method="top">0.02</axialoffset><packedlength>0.05</packedlength>'
        '<packedradius>0.014</packedradius><cd>auto</cd>'
        '<material type="surface" density="0.067">Ripstop nylon</material>'
        "<deployevent>{event}</deployevent><deploydelay>0.0</deploydelay>"
        "<diameter>0.45</diameter><linecount>6</linecount><linelength>0.4</linelength>"
        '<linematerial type="line" density="0.0025">Tubular nylon</linematerial></parachute>'
    )
    for name, event in (("Sustainer", "ejection"), ("Booster", "apogee")):
        fins = f"<subcomponents><trapezoidfinset><name>{name} fins</name>"
        assert text.count(fins) == 1
        placed = chute.format(name=name, event=event) + "<trapezoidfinset>"
        text = text.replace(fins, fins.replace("<trapezoidfinset>", placed))
    separates = "<separationevent>burnout</separationevent>"
    assert text.count(separates) == 1
    text = text.replace(separates, "<separationevent>never</separationevent>")
    path = tmp_path / "two-stage-chutes.ork"
    path.write_text(text, encoding="utf-8")
    return path


# The flight both sides fly: a site, a wind and a rail leaning off vertical, so the flight drifts.
SITE = (32.990254, -106.974998, 1400.0)
WIND = {"wind_speed_m_s": 4.0, "wind_from_deg": 250.0}
RAIL = {"rail_length_m": 1.5, "inclination_deg": 85.0, "heading_deg": 30.0}
# `hpr sim`'s recording interval when `--interval` isn't given, s.
INTERVAL_S = 0.01


def bits(value):
    return struct.pack("<d", value)


def same_bits(printed, flown, what):
    """`printed`, read from JSON, and `flown` agree in every key and every float's bits."""
    if isinstance(flown, dict):
        assert isinstance(printed, dict) and printed.keys() == flown.keys(), what
        for key in flown:
            same_bits(printed[key], flown[key], f"{what}.{key}")
    elif isinstance(flown, list):
        assert isinstance(printed, list) and len(printed) == len(flown), what
        for index, (one, other) in enumerate(zip(printed, flown)):
            same_bits(one, other, f"{what}[{index}]")
    elif isinstance(flown, float):
        assert isinstance(printed, (int, float)), what
        assert bits(float(printed)) == bits(flown), f"{what}: {printed!r} != {flown!r}"
    else:
        assert printed == flown, what


def hpr_sim(cli, design, configuration, csv_path):
    """`hpr sim` on `design` in `configuration`, flown as `Flight` flies it below, its recording
    exported to `csv_path`; its JSON output."""
    latitude, longitude, elevation = SITE
    command = [
        str(cli),
        "sim",
        str(design),
        "--offline",
        "--json",
        f"--latitude={latitude!r}",
        f"--longitude={longitude!r}",
        f"--elevation={elevation!r}",
        f"--wind={WIND['wind_speed_m_s']!r}",
        f"--wind-from={WIND['wind_from_deg']!r}",
        f"--rail-length={RAIL['rail_length_m']!r}",
        f"--inclination={RAIL['inclination_deg']!r}",
        f"--heading={RAIL['heading_deg']!r}",
        f"--interval={INTERVAL_S!r}",
        f"--export={csv_path}",
    ]
    if configuration is not None:
        command.append(f"--config={configuration}")
    done = subprocess.run(command, capture_output=True, text=True, check=False)
    assert done.returncode == 0, done.stdout + done.stderr
    return json.loads(done.stdout)


# Each case: the design, the configuration named (None for the file's default) and the devices
# that open in it, in the file's order. The level 1 design's main opens only in the first
# configuration; the two-stage design's chutes sit in different stages.
CASES = {
    "level-1-default": (level_1_with_a_main, None, ["Parachute", "Main"]),
    "level-1-second": (level_1_with_a_main, SECOND, ["Parachute"]),
    "two-stage": (two_stage_with_chutes, None, ["Sustainer chute", "Booster chute"]),
}


@pytest.mark.parametrize("case", list(CASES))
def test_a_files_recovery_flies_as_hpr_sim_flies_it_bit_for_bit(
    repo, tmp_path, hpr_cli, case, csv_header
):
    # Issue #308: `recovery=True` maps the configuration's devices as `hpr sim` does, so the
    # same file flown from the same site is the same flight, to the last bit of every number.
    make, configuration, opened = CASES[case]
    design = make(repo, tmp_path)
    exported = tmp_path / "flight.csv"
    printed = hpr_sim(hpr_cli, design, configuration, exported)
    rocket = hpr.Rocket.from_file(design, configuration, recovery=True)
    assert rocket.configuration == printed["design"]["configuration"]
    flight = hpr.Flight(rocket, hpr.Environment(*SITE, **WIND), interval_s=INTERVAL_S, **RAIL)

    # The devices: those that open, in the file's order, each at the moment the command's did;
    # one that never does is named in the same words.
    devices = printed["recovery"]
    assert [d["name"] for d in devices if d["opened_s"] is not None] == opened
    assert not any(d["added"] for d in devices)
    deployed = {e["index"]: e["time_s"] for e in flight.events if e["kind"] == "deployment"}
    assert sorted(deployed) == [i for i, d in enumerate(devices) if d["opened_s"] is not None]
    for index, time_s in deployed.items():
        assert bits(time_s) == bits(devices[index]["opened_s"]), devices[index]["name"]
    never = [note for note in printed["notes"] if "never opens" in note]
    assert [note for note in rocket.notes if "never opens" in note] == never
    assert bool(never) == (case == "level-1-second")

    # The events, each at the same time, height and vertical velocity.
    assert [(e["kind"], e["index"]) for e in printed["events"]] == [
        (e["kind"], e["index"]) for e in flight.events
    ]
    for one, other in zip(printed["events"], flight.events):
        sample = other["sample"]
        for key, value in (
            ("time_s", other["time_s"]),
            ("height_above_ground_m", sample["height_above_ground_m"]),
            ("vertical_velocity_m_s", sample["cg_velocity_enu_m_s"][2]),
        ):
            assert bits(one[key]) == bits(value), (other["kind"], key)

    # The apogee and the landing, every field.
    summary = flight.summary
    for key in ("apogee", "landing", "body_landings"):
        same_bits(printed["summary"][key], summary[key], key)

    # The recording, every cell of every row.
    with open(exported, newline="", encoding="utf-8") as file:
        header, *rows = [line.split(",") for line in file.read().splitlines()]
    assert [csv_header(name) for name in flight.columns] == header
    for index, name in enumerate(flight.columns):
        values = flight[name]
        assert len(values) == len(rows), name
        for value, row in zip(values, rows):
            assert bits(float(row[index])) == bits(value), name


def test_a_rocket_json_has_no_recovery_to_fly(repo):
    rocket = hpr.Rocket.from_file(repo / CALISTO, recovery=True)
    assert "a rocket's JSON stores no recovery device, so none of the file's flies" in rocket.notes


def test_an_hpr_document_of_an_older_version(repo):
    # A version 0.1 document, migrated as it is read, whose motor carries its own curve.
    rocket = hpr.Rocket.from_file(repo / "crates/hpr-format/fixtures/embedded-curve-0.1.hpr")
    flight = hpr.Flight(rocket, hpr.Environment(0.0, 0.0, 0.0), 1.0)
    assert flight.apogee_m > 0.0


def test_an_ork_configuration_that_does_not_fly_says_why(repo):
    # The probe's motor has no thrust curve in the file or in hpr's catalog.
    with pytest.raises(hpr.HprError, match="doesn't fly as written: .*thrust curve"):
        hpr.Rocket.from_file(repo / "validation/fixtures/ork/rod-flights/rod-5-north.ork")


def two_stage_ork(repo, tmp_path):
    """The made-up two-stage `.ork` the Rust example `ork_two_stage` holds inline."""
    example = (repo / "crates/hpr/examples/ork_two_stage.rs").read_text(encoding="utf-8")
    (document,) = re.findall(r'const ORK: &str = r#"(.*?)"#;', example, re.DOTALL)
    path = tmp_path / "two-stage.ork"
    path.write_text(document, encoding="utf-8")
    return path


def test_a_powered_separation_is_refused(repo, tmp_path):
    # hpr sim refuses it too: flown whole, the booster would ride to the ground with the
    # sustainer lit on it.
    with pytest.raises(hpr.HprError, match="separates: stage 1 drops away at 3.450 s"):
        hpr.Rocket.from_file(two_stage_ork(repo, tmp_path))


def test_an_ork_configuration_that_is_not_there(repo, tmp_path):
    with pytest.raises(hpr.HprError, match="no motor configuration `nope`; its configurations"):
        hpr.Rocket.from_file(two_stage_ork(repo, tmp_path), "nope")


def test_a_configuration_that_isnt_there(repo):
    with pytest.raises(hpr.HprError, match="no configuration `nope`"):
        hpr.Rocket.from_file(repo / CALISTO, "nope")


def test_a_file_that_isnt_there(tmp_path):
    with pytest.raises(hpr.HprError, match="missing.hpr"):
        hpr.Rocket.from_file(tmp_path / "missing.hpr")
