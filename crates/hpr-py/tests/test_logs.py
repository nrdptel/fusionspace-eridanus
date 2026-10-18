"""`FlightLog`: a flight log read by its format's own reader, a column mapping or a record layout,
the same record as `hpr analyze --record` prints, channel by channel (M7.1d2)."""

import json
import math
import shutil
import subprocess

import pytest

from fusionspace import hpr

OWN_LOGGER = "validation/fixtures/logs/own-logger"

# Each invented log, with how it is read and the file that reads it.
OWN_LOGS = [
    ("flight.csv", "mapping", "flight.mapping.json"),
    ("gps-track.csv", "mapping", "gps-track.mapping.json"),
    ("flight.bin", "layout", "flight.layout.json"),
    ("flight-capture.txt", "layout", "flight-capture.layout.json"),
]


def printed_record(hpr_cli, log, by=None, description=None, sheet=None):
    """The record `hpr analyze --record` prints, without its `tool`."""
    args = [str(hpr_cli), "analyze", str(log), "--record"]
    if by is not None:
        args += [f"--{by}", str(description)]
    if sheet is not None:
        args += ["--sheet", str(sheet)]
    run = subprocess.run(args, capture_output=True, text=True, check=False)
    assert run.returncode == 0, run.stderr
    assert run.stderr == ""
    record = json.loads(run.stdout)
    assert record.pop("tool")["name"] == "FusionSpace HPR"
    return record


def same_record(record, expected, what):
    """Two records as JSON, held equal channel by channel, then whole."""
    assert len(record["channels"]) == len(expected["channels"]), what
    for channel, wanted in zip(record["channels"], expected["channels"]):
        assert channel == wanted, f"{what}: the channel {wanted['column']!r}"
    assert record == expected, what


def gaps_as_nan(values):
    return [math.nan if value is None else value for value in values]


def same_values(array, values):
    """A NumPy array's values are the JSON list's, a gap NaN where the list has `None`."""
    assert len(array) == len(values)
    for got, wanted in zip(array.tolist(), gaps_as_nan(values)):
        assert got == wanted or (math.isnan(got) and math.isnan(wanted))


def test_an_own_loggers_files_read_as_the_command_reads_them(repo, hpr_cli):
    for log, by, name in OWN_LOGS:
        path = repo / OWN_LOGGER / log
        description = repo / OWN_LOGGER / name
        read = hpr.FlightLog.read(path, **{by: description})
        record = read.to_dict()
        same_record(record, printed_record(hpr_cli, path, by, description), log)
        # The same mapping or layout as a dict reads the same record.
        given = json.loads(description.read_text())
        assert hpr.FlightLog.read(path, **{by: given}).to_dict() == record
        assert read.format == record["format"]
        assert read.logger == "unknown"
        assert read.notes == record["notes"]
        for channel, wanted in zip(read.channels, record["channels"]):
            assert channel["column"] == wanted["column"]
            assert channel["quantity"] == wanted["quantity"]
            same_values(channel["time_s"], wanted["time_s"])
            same_values(channel["values"], wanted["values"])
        same_values(read.time_s, record["time_s"])
        same_values(read.altitude_m, record["altitude_m"])
    flight = hpr.FlightLog.read(
        repo / OWN_LOGGER / "flight.csv", mapping=repo / OWN_LOGGER / "flight.mapping.json"
    )
    # The mapping marks the altitude, in feet, as the primary track: the invented flight's
    # highest sample, 1,345.2 ft, at 10.6 s (its apogee, between samples, is 1,345.7 ft).
    assert len(flight.time_s) == 251
    assert max(flight.altitude_m) == pytest.approx(1345.2 * 0.3048, abs=1e-9)
    gps = hpr.FlightLog.read(
        repo / OWN_LOGGER / "gps-track.csv",
        mapping=str(repo / OWN_LOGGER / "gps-track.mapping.json"),
    )
    assert len(gps.time_s) == 0
    assert [c["quantity"] for c in gps.channels] == [
        "latitude",
        "longitude",
        "gnss_altitude",
        "number",
    ]
    assert [c["unit"] for c in gps.channels] == ["°", "°", "m", ""]
    assert "4 channels" in repr(gps)


def analyzed(hpr_cli, log, by=None, description=None):
    """The document `hpr analyze --json` prints."""
    args = [str(hpr_cli), "analyze", str(log), "--json"]
    if by is not None:
        args += [f"--{by}", str(description)]
    run = subprocess.run(args, capture_output=True, text=True, check=False)
    assert run.returncode == 0, run.stderr
    return json.loads(run.stdout)


READINGS = ["liftoff", "apogee", "max_speed", "max_acceleration", "landing"]
METHOD = ["source", "sample_interval_s", "step_m", "median_window_s", "peak_bound_m", "pad_altitude_m"]


def test_the_readings_are_the_commands(repo, hpr_cli):
    logs = [(log, by, repo / OWN_LOGGER / name) for log, by, name in OWN_LOGS]
    logs.append(("../synthetic-pnut.pf2", None, None))
    for log, by, description in logs:
        path = repo / OWN_LOGGER / log
        read = (
            hpr.FlightLog.read(path, **{by: description})
            if by is not None
            else hpr.FlightLog.read(path)
        )
        readings = read.readings()
        document = analyzed(hpr_cli, path, by, description)
        for name in READINGS:
            assert readings[name] == document[name], f"{log}: {name}"
        for name in METHOD:
            assert readings[name] == document["method"][name], f"{log}: {name}"
    # The invented flight's apogee is 1,345.7 ft above the pad, between samples: the reading,
    # from the barometric altitude the mapping marks, is within its bound of it.
    flight = hpr.FlightLog.read(
        repo / OWN_LOGGER / "flight.csv", mapping=repo / OWN_LOGGER / "flight.mapping.json"
    ).readings()
    apogee = flight["apogee"]
    assert apogee["status"] == "read"
    assert (apogee["source"], apogee["provenance"], apogee["method"]) == (
        "barometer",
        "measured",
        "highest_filtered",
    )
    assert abs(apogee["height_m"] - 1345.7 * 0.3048) <= apogee["bound_m"]
    # A GPS tracker's altitude gives no reading: its receiver lags a climb.
    gps = hpr.FlightLog.read(
        repo / OWN_LOGGER / "gps-track.csv", mapping=repo / OWN_LOGGER / "gps-track.mapping.json"
    ).readings()
    assert gps["source"] == "gnss"
    assert gps["apogee"] == {
        "status": "withheld",
        "reason": "receiver_lag",
        "detail": gps["apogee"]["detail"],
    }


def test_a_log_read_by_its_own_reader_is_the_commands(repo, hpr_cli):
    path = repo / "validation/fixtures/logs/synthetic-pnut.pf2"
    read = hpr.FlightLog.read(path)
    same_record(read.to_dict(), printed_record(hpr_cli, path), path.name)
    assert read.logger == "PerfectFlite Pnut"
    assert read.format == "perfect_flite_pf2"
    assert read.stated["apogee_m"] is not None


def test_the_header_mapping_is_the_one_the_command_prints(repo, hpr_cli):
    path = repo / OWN_LOGGER / "flight.csv"
    run = subprocess.run(
        [str(hpr_cli), "analyze", str(path), "--print-mapping", "--json"],
        capture_output=True,
        text=True,
        check=False,
    )
    assert run.returncode == 0, run.stderr
    mapping = hpr.FlightLog.header_mapping(path)
    assert mapping == json.loads(run.stdout)
    assert "primary" not in mapping
    # Edited as the guide edits it, the header's mapping is the one the folder holds.
    mapping["primary"] = 1
    mapping["columns"][1]["quantity"] = "altitude_above_ground"
    mapping["columns"][1]["unit"] = "ft"
    assert mapping == json.loads((repo / OWN_LOGGER / "flight.mapping.json").read_text())


def test_a_mapping_or_layout_is_refused_naming_the_field(repo, tmp_path):
    path = repo / OWN_LOGGER / "flight.csv"
    mapping = json.loads((repo / OWN_LOGGER / "flight.mapping.json").read_text())
    mapping["columns"][1]["unit"] = 3
    with pytest.raises(hpr.HprError, match=r"`columns\[1\]\.unit`: invalid type: integer `3`"):
        hpr.FlightLog.read(path, mapping=mapping)
    mapping["columns"][1]["unit"] = "ft"
    mapping["primary"] = 2
    with pytest.raises(hpr.HprError, match=r"the primary track, \"Pressure \(Pa\)\", is mapped"):
        hpr.FlightLog.read(path, mapping=mapping)
    layout = json.loads((repo / OWN_LOGGER / "flight.layout.json").read_text())
    layout["fields"][1]["kind"] = "float"
    with pytest.raises(hpr.HprError, match=r"`fields\[1\]\.kind`: unknown variant `float`"):
        hpr.FlightLog.read(repo / OWN_LOGGER / "flight.bin", layout=layout)
    with pytest.raises(hpr.HprError, match="not both"):
        hpr.FlightLog.read(path, mapping={}, layout={})
    with pytest.raises(hpr.HprError, match="no table is read by a mapping"):
        hpr.FlightLog.read(path, sheet=0)
    with pytest.raises(hpr.HprError, match="a dict, or the path of its JSON file"):
        hpr.FlightLog.read(path, mapping=5)
    with pytest.raises(hpr.HprError, match="must be one JSON can hold"):
        hpr.FlightLog.read(path, mapping={"primary": float("nan")})
    with pytest.raises(hpr.HprError, match="must be one JSON can hold"):
        hpr.FlightLog.read(path, mapping={"primary": object()})
    # A mapping file Windows PowerShell 5.1 wrote, UTF-16 with its mark, reads as UTF-8 does.
    text = (repo / OWN_LOGGER / "flight.mapping.json").read_text()
    utf16 = tmp_path / "utf16.json"
    utf16.write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
    assert (
        hpr.FlightLog.read(path, mapping=utf16).to_dict()
        == hpr.FlightLog.read(path, mapping=repo / OWN_LOGGER / "flight.mapping.json").to_dict()
    )
    # By its own reader, the table reads by what its header states: the altitude, whose unit it
    # doesn't state, kept as written.
    by_header = hpr.FlightLog.read(path)
    assert by_header.format == "generic_table"
    assert len(by_header.time_s) == 0
    assert any('"Alt"' in note for note in by_header.notes)


def test_refs_mapped_tables_and_layout_files_read_the_same_on_every_surface(repo, hpr_cli):
    """Each sample under refs/ that a committed mapping or layout reads: the library's record, as
    `cargo xtask logs --surfaces` writes it once `hpr_flightdata::custom` reads it as the stated
    reading does, is the command's and Python's, channel by channel. The samples are other
    people's data, so CI has none and this skips there."""
    if not (repo / "refs" / "hpr-sim-fixtures").is_dir():
        pytest.skip("no refs/hpr-sim-fixtures: the private samples aren't here")
    cargo = shutil.which("cargo")
    assert cargo, "cargo writes the library's records (`cargo xtask logs --surfaces`)"
    folder = repo / "refs" / "surfaces"
    run = subprocess.run(
        [cargo, "xtask", "logs", "--surfaces", "refs/surfaces"],
        cwd=repo,
        capture_output=True,
        text=True,
        check=False,
    )
    assert run.returncode == 0, run.stderr[-4000:]
    manifest = json.loads((folder / "manifest.json").read_text())
    assert manifest["refs"]
    files = manifest["files"]
    # validation/log-mappings.json's 6, log-held-mappings.json's 20, log-layouts.json's 5.
    assert len(files) == 31
    assert sum(1 for f in files if f["by"] == "layout") == 5
    for entry in files:
        what = entry["id"]
        library = json.loads((folder / entry["record"]).read_text())
        description = folder / entry["description"]
        by = entry["by"]
        kwargs = {by: description}
        if entry["sheet"] is not None:
            kwargs["sheet"] = entry["sheet"]
        read = hpr.FlightLog.read(entry["log"], **kwargs)
        same_record(read.to_dict(), library, f"{what}, Python")
        same_record(
            printed_record(hpr_cli, entry["log"], by, description, entry["sheet"]),
            library,
            f"{what}, the command",
        )
