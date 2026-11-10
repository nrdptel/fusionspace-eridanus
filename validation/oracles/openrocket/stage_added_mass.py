"""Whether a motor aboard, or pods, move the mass OpenRocket 24.12 adds for a stage's override.

A stage's mass override that doesn't cover the stage's parts is added to what the parts weigh
(`conventions.py`'s stage probes, ADR-335). With no center of gravity stated, OpenRocket puts
that mass at the center of the stage's parts. Those probes carry no motor, so they can't say
whether the center is the parts' alone or the parts' and the motor's together, which would move
the added mass as the propellant burns.

This script asks. It takes the pod probes' airframe with no pods (`pod_probes.py`: a nose with
ballast, a tube, three fins and an AeroTech H128W in a mount), gives its stage each override
below, and records OpenRocket's mass, center of mass and inertias three ways: the structure
alone, at launch with the motor full, and at burnout with the motor spent.

- `no override`: the airframe as it is;
- `a mass override on the stage, not its parts`: 2 kg, its flag false;
- `both overrides on the stage, not its parts`: 2 kg at 0.4 m, both flags false;
- `a mass override on the stage, not its parts, and a center that covers them`: 2 kg, its flag
  false, and a center of gravity of 0.4 m whose flag is true.

hpr's test `hpr_validate::openrocket::tests` reads the record: the motor is what launch adds to
the structure with no override, and with each override OpenRocket's launch and burnout are that
same motor added to the overridden structure. So the added mass stays where the structure puts
it, motor or no motor.

It also asks whether a stage's pods are among "the stage's parts", and where the added mass sits
when the parts' center is off the axis. The same airframe carries three pods around its tube
(`pod_probes.py`'s `pods-bodies-3`), or one pod alone, which pulls the parts' center off the axis:

- `three pods and no override`, `three pods and a mass override on the stage, not its parts`;
- `one pod and no override`, and one pod under each of the three overrides above.

Three pods move the parts' center aft, so the record says whether the added mass sits at the
center with the pods counted. One pod moves it sideways too, so the record says whether an added
mass at a station sits on the axis or level with the parts' center.

OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The class and
method names used are the public API that `javap` prints for the jar. OpenRocket 24.12 needs Java
17 exactly; see `automatic_radius.py`. Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/stage_added_mass.py \\
        validation/fixtures/ork/openrocket-stage-added-mass.json

The fixture is written to the path given, not to standard output, which OpenRocket logs to.
"""

import hashlib
import json
import logging
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import automatic_radius  # noqa: E402 - the jar's path, the loader and the settling save
import conventions  # noqa: E402 - the override tags and the fixed ids
import events  # noqa: E402 - the JVM start with the motor database, which a motor needs
import pod_probes  # noqa: E402 - the airframe and its motor

# Provenance written into the fixture (docs/VALIDATION.md). Update when regenerating.
GENERATED = "2026-11-10"

THREE_PODS = pod_probes.pod_set(
    3, "0.01", "0.0", "0.25", pod_probes.pod_nose(21, "0.012") + pod_probes.pod_tube(22, "0.012", "0.25")
)
ONE_POD = pod_probes.pod_set(
    1, "0.01", "0.0", "0.25", pod_probes.pod_nose(21, "0.012") + pod_probes.pod_tube(22, "0.012", "0.25")
)

#: Each probe on the airframe with pods: its pods, then its stage's override tags.
POD_PROBES = {
    "three pods and no override": (THREE_PODS, ""),
    "three pods and a mass override on the stage, not its parts": (
        THREE_PODS,
        conventions.overrides(mass_kg=2.0, children_mass="false"),
    ),
    "one pod and no override": (ONE_POD, ""),
    "one pod and a mass override on the stage, not its parts": (
        ONE_POD,
        conventions.overrides(mass_kg=2.0, children_mass="false"),
    ),
    "one pod and both overrides on the stage, not its parts": (
        ONE_POD,
        conventions.overrides(2.0, 0.4, "false", "false"),
    ),
    "one pod and a mass override on the stage, not its parts, and a center that covers them": (
        ONE_POD,
        conventions.overrides(2.0, 0.4, "false", "true"),
    ),
}

PROBES = {
    "no override": "",
    "a mass override on the stage, not its parts": conventions.overrides(
        mass_kg=2.0, children_mass="false"
    ),
    "both overrides on the stage, not its parts": conventions.overrides(2.0, 0.4, "false", "false"),
    "a mass override on the stage, not its parts, and a center that covers them": conventions.overrides(
        2.0, 0.4, "false", "true"
    ),
}


def document(stage_tags, pods=""):
    """The pod probes' airframe with `pods` (none by default), its stage carrying `stage_tags`."""
    stage_id = f"<id>{conventions.uid(99)}</id>"
    text = pod_probes.document("Probe", pods)
    if text.count(stage_id) != 1:
        sys.exit("the stage's id is not written exactly once in the probe airframe")
    return text.replace(stage_id, stage_id + stage_tags)


def weighed(body):
    """One of OpenRocket's rigid bodies: mass, center of mass and inertias."""
    return {
        "mass_kg": float(body.getMass()),
        "cm_x_m": float(body.getCM().x),
        "cm_y_m": float(body.getCM().y),
        "cm_z_m": float(body.getCM().z),
        "ixx": float(body.getIxx()),
        "iyy": float(body.getIyy()),
        "izz": float(body.getIzz()),
    }


def measure(text, scratch, name):
    from info.openrocket.core.masscalc import MassCalculator

    path = Path(scratch) / f"{name}.ork"
    path.write_text(text, encoding="utf-8")
    document_ = automatic_radius.load(path)
    automatic_radius.saved_radii(document_)
    configuration = document_.getRocket().getSelectedConfiguration()
    motors = [
        str(held.getMotor().getDesignation())
        for held in configuration.getActiveMotors()
        if held.getMotor() is not None
    ]
    if motors != ["H128W"]:
        sys.exit(f"{name}: OpenRocket flies {motors}, not the one H128W the probe names")
    return {
        "document": text,
        "motors": motors,
        "structure": weighed(MassCalculator.calculateStructure(configuration)),
        "launch": weighed(MassCalculator.calculateLaunch(configuration)),
        "burnout": weighed(MassCalculator.calculateBurnout(configuration)),
    }


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: stage_added_mass.py OUTPUT.json")
    output = Path(sys.argv[1])
    logging.disable(logging.CRITICAL)
    events.start()
    import jpype
    from info.openrocket.core.util import BuildProperties
    from java.lang import System

    probes = {}
    with tempfile.TemporaryDirectory() as scratch:
        for k, (question, tags) in enumerate(PROBES.items()):
            probes[question] = measure(document(tags), scratch, f"probe-{k}")
        for k, (question, (pods, tags)) in enumerate(POD_PROBES.items()):
            probes[question] = measure(document(tags, pods), scratch, f"pod-probe-{k}")

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(
        json.dumps(
            {
                "source": "validation/oracles/openrocket/stage_added_mass.py",
                "command": " ".join(
                    ["refs/venv/bin/python", "validation/oracles/openrocket/stage_added_mass.py"]
                    + sys.argv[1:]
                ),
                "generated": GENERATED,
                "inputs_sha256": {
                    name: hashlib.sha256(Path(module.__file__).read_bytes()).hexdigest()
                    for name, module in [
                        ("stage_added_mass.py", sys.modules[__name__]),
                        ("pod_probes.py", pod_probes),
                        ("conventions.py", conventions),
                        ("events.py", events),
                        ("automatic_radius.py", automatic_radius),
                    ]
                },
                "openrocket": str(BuildProperties.getVersion()),
                "jar_sha256": hashlib.sha256(automatic_radius.JAR.read_bytes()).hexdigest(),
                "java": str(System.getProperty("java.version")),
                "jpype": jpype.__version__,
                "probes": probes,
            },
            indent=1,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
