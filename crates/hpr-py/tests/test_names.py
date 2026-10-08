"""The package's names (ADR-199 §2): the distribution `fusionspace-hpr`, imported as
`fusionspace.hpr`, with `fusionspace` an implicit namespace package other FusionSpace packages can
share."""

import importlib.metadata
import importlib.util

import fusionspace
from fusionspace import hpr


def test_the_distribution_is_fusionspace_hpr():
    distribution = importlib.metadata.distribution("fusionspace-hpr")
    assert distribution.version == hpr.__version__
    assert importlib.util.find_spec("hpr") is None, "nothing installs a top-level `hpr`"


def test_fusionspace_is_a_namespace_package():
    # PEP 420: a namespace package has no __init__.py, so no __file__, and a path of its own kind.
    assert getattr(fusionspace, "__file__", None) is None
    assert type(fusionspace.__path__).__name__ == "_NamespacePath"
    assert hpr.__name__ == "fusionspace.hpr"


def test_every_type_names_the_package_as_its_module():
    for name in ["DragTable", "Environment", "Flight", "HprError", "MonteCarlo", "Motor", "Rocket"]:
        assert getattr(hpr, name).__module__ == "fusionspace.hpr", name
