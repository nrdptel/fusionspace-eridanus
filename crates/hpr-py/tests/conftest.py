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
