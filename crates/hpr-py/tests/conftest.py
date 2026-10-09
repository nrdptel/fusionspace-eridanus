"""Shared paths for the tests: the repository's public files they read, and the command line some
of them compare the package against."""

import sys
from pathlib import Path

import pytest

REPO = Path(__file__).resolve().parents[3]


@pytest.fixture
def repo() -> Path:
    """The repository's root."""
    return REPO


@pytest.fixture(scope="session")
def hpr_cli() -> Path:
    """The release build of the command line, as `scripts/python-tests.sh` builds it: the wheel is
    a release build too, and a debug build's last digits can differ from a release build's."""
    name = "hpr.exe" if sys.platform == "win32" else "hpr"
    path = REPO / "target" / "release" / name
    assert path.is_file(), (
        f"{path} isn't built: scripts/python-tests.sh builds it, or "
        "`cargo build --release --locked -p fusionspace-hpr-cli`"
    )
    return path


# How a CSV header writes a column's unit, in brackets after its words: the last one or two of
# the name's `_`-separated words, two-word units first.
UNITS = [
    (["m", "s2"], "m/s^2"),
    (["m", "s"], "m/s"),
    (["rad", "s"], "rad/s"),
    (["m2"], "m^2"),
    (["pa"], "Pa"),
    (["kg"], "kg"),
    (["n"], "N"),
    (["m"], "m"),
    (["s"], "s"),
    (["rad"], "rad"),
    (["deg"], "deg"),
    (["cal"], "cal"),
]


def header_of(name):
    """A column's name as a CSV header writes it: `apogee [m]` for `apogee_m`."""
    words = name.split("_")
    for unit, written in UNITS:
        if len(words) > len(unit) and words[-len(unit) :] == unit:
            return f"{' '.join(words[: -len(unit)])} [{written}]"
    return " ".join(words)


@pytest.fixture
def csv_header():
    """How a CSV header writes a column's name, written out here apart from the Rust rule
    (`hpr_sim::export::csv_header`) so the two check each other."""
    return header_of
