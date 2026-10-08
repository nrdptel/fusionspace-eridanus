"""FusionSpace HPR · Sim, a flight simulator for hobby and high-power rockets.

Four types make a flight, in the order a program needs them: an ``Environment``, a ``Motor``, a
``Rocket`` and a ``Flight``; a ``MonteCarlo`` run flies one many times, its inputs scattered.
Values are in SI units, named in every argument and attribute. The guide's Python page walks
through them: https://hpr.fusionspace.co/python.html

Import it as ``from fusionspace import hpr``.
"""

from fusionspace.hpr._hpr import (
    DragTable,
    Environment,
    Flight,
    HprError,
    MonteCarlo,
    Motor,
    Rocket,
    __version__,
    materials,
)

__all__ = [
    "DragTable",
    "Environment",
    "Flight",
    "HprError",
    "MonteCarlo",
    "Motor",
    "Rocket",
    "__version__",
    "materials",
]
