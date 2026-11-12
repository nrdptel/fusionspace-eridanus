"""Where OpenRocket 24.12 puts a stage's added mass beside a parallel stage, and on parts that
weigh nothing.

A stage's mass override that doesn't cover the stage's parts is added to what the parts weigh
(`conventions.py`'s stage probes and `stage_added_mass.py`, ADR-335). Those probes have no
parallel stage, and one stage of parts that weigh nothing, which starts at the rocket's tip. So
they can't say where the mass sits on a parallel stage, on the stage one hangs on, or on a lower
stage that weighs nothing. This script asks (M2.3d8, ADR-341).

**Boosters.** The pod probes' airframe with no motor (`pod_probes.py`: a nose with ballast, a
tube and three fins) carries a parallel stage of three boosters round its tube, or of one booster
alone, which pulls the center off the axis. Each booster is the pod probes' nose and tube. The
overrides are 2 kg and, on the parallel stage, a center 0.1 m aft of its forward end, or on the
stage the boosters hang on, 0.4 m aft of the tip. Each is asked with three boosters and with one:

- no override;
- on the parallel stage: the mass alone, not covering its parts; both, neither covering; the mass
  not covering beside a center that covers; the mass covering; both covering; the mass covering
  beside a center that doesn't; the center alone, not covering and covering;
- on the stage the boosters hang on: the mass alone, not covering; both, neither covering; the
  mass not covering beside a center that covers; the mass covering; both covering; the center
  alone, covering;
- on each stage at once: the mass alone, not covering; and both, neither covering.

With three boosters the parallel stage is also asked with no flag written: the mass alone, and
both overrides. And three boosters that weigh nothing (no wall) carry the mass alone, not
covering.

Three probes carry the airframe's motor, an AeroTech H128W, and drop their boosters half a
second after launch, so that HPR Sim can fly them: three boosters with no override, with the
mass alone on the parallel stage, and with both overrides on it, neither covering. They also
record OpenRocket's rocket at launch, the motor full, and at burnout, the motor spent, the
boosters still on (`launch`, `burnout`), which says whether the motor moves the added mass.

**Stages that weigh nothing.** The conventions probe's nose and tube above its tube again, as two
stages, one or both with no wall: the lower one, the upper one or both carry the 2 kg mass that
doesn't cover the parts, alone, beside a center 0.4 m aft that doesn't cover them either, or
beside one that does.

Each probe records OpenRocket's structure three ways: every stage active (`structure`), and each
stage active alone (`alone`, in the order OpenRocket numbers the stages: the axial ones from the
tip, a parallel stage after the stage it hangs on). `alone` is the stage as OpenRocket weighs it
once the others are gone.

OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The class and
method names used are the public API that `javap` prints for the jar. OpenRocket 24.12 needs Java
17 exactly; see `automatic_radius.py`. Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/added_mass_placement.py \\
        validation/fixtures/ork/openrocket-added-mass-placement.json

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
import conventions  # noqa: E402 - the override tags, the fixed ids and the two-stage parts
import events  # noqa: E402 - the JVM start with the motor database, which a motor needs
import pod_probes  # noqa: E402 - the airframe, its motor and a booster's parts

# Provenance written into the fixture (docs/VALIDATION.md). Update when regenerating.
GENERATED = "2026-11-11"

uid = conventions.uid
overrides = conventions.overrides

#: The parallel stage's forward end is 0.25 m below the top of the airframe's tube.
BOOSTER_TOP = "0.25"


def booster_parts(wall="0.001"):
    """A booster: the pod probes' nose and tube, with a 1 mm wall unless `wall` says otherwise."""
    parts = pod_probes.pod_nose(21, "0.012") + pod_probes.pod_tube(22, "0.012", "0.25")
    old = "<thickness>0.001</thickness>"
    if parts.count(old) != 2:
        sys.exit("a booster's two walls are not written as expected")
    return parts.replace(old, f"<thickness>{wall}</thickness>")


#: When the boosters of a probe that flies drop: half a second after launch.
DROPPED = "<separationevent>launch</separationevent><separationdelay>0.5</separationdelay>"


def boosters(count, tags="", wall="0.001", separation=""):
    """A parallel stage of `count` boosters, 10 mm out from the airframe's surface, carrying the
    override `tags` and the `separation` tags."""
    return (
        f"<parallelstage><name>Boosters</name><id>{uid(20)}</id>{tags}{separation}"
        f"<instancecount>{count}</instancecount>"
        '<radiusoffset method="relative">0.01</radiusoffset>'
        '<angleoffset method="relative">0.0</angleoffset>'
        f'<axialoffset method="top">{BOOSTER_TOP}</axialoffset>'
        f'<position type="top">{BOOSTER_TOP}</position>'
        f"<subcomponents>{booster_parts(wall)}</subcomponents></parallelstage>"
    )


def airframe(count, core_tags="", booster_tags="", wall="0.001", motor=False):
    """The pod probes' airframe, its stage carrying `core_tags`, with `count` boosters carrying
    `booster_tags`. With `motor` it holds its motor and drops the boosters (`DROPPED`)."""
    stage_id = f"<id>{uid(99)}</id>"
    hung = boosters(count, booster_tags, wall, DROPPED if motor else "")
    text = pod_probes.document("Probe", hung, motor=motor)
    if text.count(stage_id) != 1:
        sys.exit("the stage's id is not written exactly once in the probe airframe")
    return text.replace(stage_id, stage_id + core_tags)


def two_stages(upper_tags, lower_tags, upper_weighs, lower_weighs):
    """The conventions probe's nose and tube as one stage above its tube again as another, a
    stage that doesn't weigh anything written with no wall."""
    upper = "0.002" if upper_weighs else "0.0"
    lower = conventions.tube(thickness="0.002" if lower_weighs else "0.0")
    lower = lower.replace(uid(2), uid(95)).replace("<name>Tube</name>", "<name>Lower tube</name>")
    return (
        "<?xml version='1.0' encoding='utf-8'?>\n"
        '<openrocket version="1.10" creator="hpr-sim conventions probe">'
        f"<rocket><name>Probe</name><id>{uid(98)}</id><subcomponents>"
        f"<stage><name>Upper stage</name><id>{uid(99)}</id>{upper_tags}<subcomponents>"
        f"{conventions.nose(thickness=upper)}{conventions.tube(thickness=upper)}"
        "</subcomponents></stage>"
        f"<stage><name>Lower stage</name><id>{uid(96)}</id>{lower_tags}<subcomponents>"
        f"{lower}</subcomponents></stage></subcomponents></rocket></openrocket>\n"
    )


ADDED = overrides(mass_kg=2.0, children_mass="false")
WHOLE = overrides(mass_kg=2.0, children_mass="true")

#: Each override on the parallel stage: what it asks, and its tags.
ON_BOOSTERS = {
    "a mass override on the parallel stage, not its parts": ADDED,
    "both overrides on the parallel stage, not its parts": overrides(2.0, 0.1, "false", "false"),
    "a mass override on the parallel stage, not its parts, and a center that covers them": overrides(
        2.0, 0.1, "false", "true"
    ),
    "a mass override on the parallel stage": WHOLE,
    "both overrides on the parallel stage": overrides(2.0, 0.1, "true", "true"),
    "a mass override that covers the parallel stage's parts and a center that doesn't": overrides(
        2.0, 0.1, "true", "false"
    ),
    "a center of gravity override on the parallel stage, not its parts": overrides(
        cg_m=0.1, children_cg="false"
    ),
    "a center of gravity override on the parallel stage": overrides(cg_m=0.1, children_cg="true"),
}

#: Each override on the stage the boosters hang on.
ON_CORE = {
    "a mass override on the stage they hang on, not its parts": ADDED,
    "both overrides on the stage they hang on, not its parts": overrides(
        2.0, 0.4, "false", "false"
    ),
    "a mass override on the stage they hang on, not its parts, and a center that covers them": overrides(
        2.0, 0.4, "false", "true"
    ),
    "a mass override on the stage they hang on": WHOLE,
    "both overrides on the stage they hang on": overrides(2.0, 0.4, "true", "true"),
    "a center of gravity override on the stage they hang on": overrides(
        cg_m=0.4, children_cg="true"
    ),
}

#: Each probe: what it asks, and its document.
PROBES = {}
for _count, _boosters in [(3, "three boosters"), (1, "one booster")]:
    PROBES[f"{_boosters} and no override"] = airframe(_count)
    for _question, _tags in ON_BOOSTERS.items():
        PROBES[f"{_boosters} and {_question}"] = airframe(_count, booster_tags=_tags)
    for _question, _tags in ON_CORE.items():
        PROBES[f"{_boosters} and {_question}"] = airframe(_count, core_tags=_tags)
    PROBES[f"{_boosters} and a mass override on each stage, not its parts"] = airframe(
        _count, ADDED, ADDED
    )
    PROBES[f"{_boosters} and both overrides on each stage, not its parts"] = airframe(
        _count, overrides(2.0, 0.4, "false", "false"), overrides(2.0, 0.1, "false", "false")
    )
PROBES["three boosters and a mass override on the parallel stage with no flag"] = airframe(
    3, booster_tags=overrides(mass_kg=2.0)
)
PROBES["three boosters and both overrides on the parallel stage with no flag"] = airframe(
    3, booster_tags=overrides(2.0, 0.1)
)
PROBES[
    "three boosters that weigh nothing and a mass override on the parallel stage, not its parts"
] = airframe(3, booster_tags=ADDED, wall="0.0")

#: The probes that carry the airframe's motor.
WITH_A_MOTOR = {
    "three boosters, a motor in the airframe and no override": airframe(3, motor=True),
    "three boosters, a motor in the airframe and a mass override on the parallel stage, not its parts": airframe(
        3, booster_tags=ADDED, motor=True
    ),
    "three boosters, a motor in the airframe and both overrides on the parallel stage, not its parts": airframe(
        3, booster_tags=overrides(2.0, 0.1, "false", "false"), motor=True
    ),
}
PROBES.update(WITH_A_MOTOR)

_BOTH = overrides(2.0, 0.4, "false", "false")
_BESIDE = overrides(2.0, 0.4, "false", "true")
PROBES.update(
    {
        "a lower stage that weighs nothing, with no override": two_stages("", "", True, False),
        "a lower stage that weighs nothing, a mass override on it, not its parts": two_stages(
            "", ADDED, True, False
        ),
        "a lower stage that weighs nothing, both overrides on it, not its parts": two_stages(
            "", _BOTH, True, False
        ),
        "a lower stage that weighs nothing, a mass override on it, not its parts, and a center that covers them": two_stages(
            "", _BESIDE, True, False
        ),
        "an upper stage that weighs nothing, a mass override on it, not its parts": two_stages(
            ADDED, "", False, True
        ),
        "an upper stage that weighs nothing, a mass override on it, not its parts, and a center that covers them": two_stages(
            _BESIDE, "", False, True
        ),
        "two stages that weigh nothing, a mass override on the lower, not its parts": two_stages(
            "", ADDED, False, False
        ),
        "two stages that weigh nothing, a mass override on the upper, not its parts": two_stages(
            ADDED, "", False, False
        ),
        "two stages that weigh nothing, a mass override on each, not its parts": two_stages(
            ADDED, ADDED, False, False
        ),
    }
)


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


def measure(text, scratch, name, motor):
    from info.openrocket.core.masscalc import MassCalculator

    path = Path(scratch) / f"{name}.ork"
    path.write_text(text, encoding="utf-8")
    document_ = automatic_radius.load(path)
    automatic_radius.saved_radii(document_)
    configuration = document_.getRocket().getSelectedConfiguration()
    count = int(configuration.getStageCount())
    if count != 2:
        sys.exit(f"{name}: OpenRocket counts {count} stages, not the two the probe writes")
    configuration.setAllStages()
    structure = weighed(MassCalculator.calculateStructure(configuration))
    alone = []
    for stage in range(count):
        configuration.setOnlyStage(stage)
        alone.append(weighed(MassCalculator.calculateStructure(configuration)))
    configuration.setAllStages()
    again = weighed(MassCalculator.calculateStructure(configuration))
    if again != structure:
        sys.exit(f"{name}: the structure changed after each stage was weighed alone")
    measured = {"document": text, "structure": structure, "alone": alone}
    motors = [
        str(held.getMotor().getDesignation())
        for held in configuration.getActiveMotors()
        if held.getMotor() is not None
    ]
    if motors != (["H128W"] if motor else []):
        sys.exit(f"{name}: OpenRocket flies {motors}, not what the probe names")
    if motor:
        measured["motors"] = motors
        measured["launch"] = weighed(MassCalculator.calculateLaunch(configuration))
        measured["burnout"] = weighed(MassCalculator.calculateBurnout(configuration))
    return measured


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: added_mass_placement.py OUTPUT.json")
    output = Path(sys.argv[1])
    logging.disable(logging.CRITICAL)
    events.start()
    import jpype
    from info.openrocket.core.util import BuildProperties
    from java.lang import System

    probes = {}
    with tempfile.TemporaryDirectory() as scratch:
        for k, (question, text) in enumerate(PROBES.items()):
            probes[question] = measure(text, scratch, f"probe-{k}", question in WITH_A_MOTOR)

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(
        json.dumps(
            {
                "source": "validation/oracles/openrocket/added_mass_placement.py",
                "command": " ".join(
                    [
                        "refs/venv/bin/python",
                        "validation/oracles/openrocket/added_mass_placement.py",
                    ]
                    + sys.argv[1:]
                ),
                "generated": GENERATED,
                "inputs_sha256": {
                    name: hashlib.sha256(Path(module.__file__).read_bytes()).hexdigest()
                    for name, module in [
                        ("added_mass_placement.py", sys.modules[__name__]),
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
