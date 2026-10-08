"""When OpenRocket 24.12 counts a stage of several motors as burnt out.

A `.ork` can separate a stage at its own `burnout` and light the stage above at the `burnout` of
the stage below ("first burnout of previous stage", as OpenRocket labels it). A stage whose motors
sit in several mounts, a main tube and side pods, burns them out at different times, and nothing
published says which of those times the events take: the earliest burnout in the stage, the
latest, or the burnout of one particular mount. This script measures it on the jar's
*Pods--powered with recovery deployment* example, first configuration: the sustainer's C6-7 lit at
the booster's burnout; the booster's B6-0 in its main motor tube and an A3-4 in each of two pod
motor tubes, all lit at launch; the booster separating at its burnout. It flies

- `as-written`: the configuration as the example has it;
- `main-burns-longest`: the booster's main tube given the sustainer's C6 (the same 18 mm), with no
  ejection delay and lit at launch, so the pods' A3s burn out first and the main tube last;
- `pods-burn-longest`: the main tube's B6 kept and each pod given that C6 instead, so the main
  tube burns out first and the pods last. A C6 is wider than the pods' 13 mm tubes; OpenRocket
  flies the motor it is given, and this probe asks only which burnout the events follow.

Each is flown in a new simulation in calm air, with OpenRocket's default launch conditions and
nothing deployed. The record holds, for each flight, every mount's motor (designation, stage,
ignition event, ejection delay, the burn time OpenRocket's motor reports and the time of its last
thrust point), the branch count, and for every branch its name and each event's kind, time and the
name of the component OpenRocket gives as its source. For the first branch it also holds the
altitude and total velocity at the row nearest the stage separation, the last row, and the cause
of any abort.

OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The class and
method names used are the public API that `javap` prints for the jar. Only numbers and the names
of OpenRocket's events and of the example's components are recorded. OpenRocket 24.12 needs Java
17 exactly; see `automatic_radius.py`. Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/first_burnout.py \\
        validation/fixtures/ork/openrocket-first-burnout.json

The fixture is written to the path given, not to standard output, which OpenRocket logs to.
"""

import hashlib
import json
import logging
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import automatic_radius  # noqa: E402 - the jar's path
import events  # noqa: E402 - the JVM start, the example loader and the component walk

GENERATED = "2026-10-05"
SEED = 1
EXAMPLE = "datafiles/examples/Pods--powered with recovery deployment.ork"
CONFIGURATION = 0
PROBES = ["as-written", "main-burns-longest", "pods-burn-longest"]


def in_pod(component):
    """Whether `component` sits inside a pod set."""
    from info.openrocket.core.rocketcomponent import PodSet

    parent = component.getParent()
    while parent is not None:
        if isinstance(parent, PodSet):
            return True
        parent = parent.getParent()
    return False


def nearest(times, time_s):
    """The index of the row whose time is nearest `time_s`."""
    return min(range(len(times)), key=lambda i: abs(times[i] - time_s))


def flight(probe):
    """The example's first configuration, changed as `probe` says, flown with nothing deployed."""
    from info.openrocket.core.document import Simulation
    from info.openrocket.core.motor import IgnitionEvent
    from info.openrocket.core.rocketcomponent import (
        DeploymentConfiguration,
        MotorMount,
        RecoveryDevice,
    )
    from info.openrocket.core.simulation import FlightDataType

    document = events.example(EXAMPLE)
    rocket = document.getRocket()
    configuration = rocket.getFlightConfigurationByIndex(
        CONFIGURATION
    ).getFlightConfigurationID()
    mounts = []
    for mount in events.components(rocket, MotorMount):
        if not mount.isMotorMount():
            continue
        held = mount.getMotorConfig(configuration)
        if held.getMotor() is not None:
            mounts.append((mount, held))
    sustainer = [h for m, h in mounts if int(m.getStage().getStageNumber()) == 0]
    main = [h for m, h in mounts if int(m.getStage().getStageNumber()) == 1 and not in_pod(m)]
    pods = [h for m, h in mounts if int(m.getStage().getStageNumber()) == 1 and in_pod(m)]
    if len(sustainer) != 1 or len(main) != 1 or not pods:
        sys.exit("the example's first configuration is not one sustainer motor, a main booster "
                 "motor and pod motors")
    c6 = sustainer[0].getMotor()
    changed = {"main-burns-longest": main, "pods-burn-longest": pods}.get(probe, [])
    for held in changed:
        held.setMotor(c6)
        held.setEjectionDelay(0.0)
        held.setIgnitionEvent(IgnitionEvent.LAUNCH)
        held.setIgnitionDelay(0.0)
    for device in events.components(rocket, RecoveryDevice):
        held = device.getDeploymentConfigurations().get(configuration)
        held.setDeployEvent(DeploymentConfiguration.DeployEvent.NEVER)
    booster_stage = main[0].getMount().getStage()
    separation = booster_stage.getSeparationConfigurations().get(configuration)

    simulation = Simulation(document, rocket)
    simulation.setFlightConfigurationId(configuration)
    options = simulation.getOptions()
    options.setWindSpeedAverage(0.0)
    options.setWindTurbulenceIntensity(0.0)
    options.setRandomSeed(SEED)
    simulation.simulate()
    data = simulation.getSimulatedData()

    def motor(mount, held):
        engine = held.getMotor()
        last_thrust_s = None
        if hasattr(engine, "getTimePoints"):
            times = [float(t) for t in engine.getTimePoints()]
            thrusts = [float(f) for f in engine.getThrustPoints()]
            lit = [t for t, f in zip(times, thrusts) if f > 0.0]
            last_thrust_s = lit[-1] if lit else None
        return {
            "mount": str(mount.getName()),
            "in_pod": in_pod(mount),
            "stage": int(mount.getStage().getStageNumber()),
            "instances": int(held.getMotorCount()),
            "designation": str(engine.getDesignation()),
            "ignition_event": events.name_of(held.getIgnitionEvent()),
            "ejection_delay_s": float(held.getEjectionDelay()),
            "burn_time_s": float(engine.getBurnTime()),
            "burn_time_estimate_s": float(engine.getBurnTimeEstimate()),
            "last_thrust_point_s": last_thrust_s,
        }

    def event(e):
        source = e.getSource()
        found = {
            "type": events.name_of(e.getType()),
            "time_s": float(e.getTime()),
            "source": None if source is None else str(source.getName()),
        }
        if found["type"] == "SIM_ABORT" and e.getData() is not None:
            data_ = e.getData()
            found["abort"] = (
                events.name_of(data_.getCause()) if hasattr(data_, "getCause") else str(data_)
            )
        return found

    branches = []
    for i in range(int(data.getBranchCount())):
        branch = data.getBranch(i)
        branches.append(
            {"name": str(branch.getName()), "events": [event(e) for e in branch.getEvents()]}
        )
    first = data.getBranch(0)
    times = [float(v) for v in first.get(FlightDataType.TYPE_TIME)]
    altitude = [float(v) for v in first.get(FlightDataType.TYPE_ALTITUDE)]
    speed = [float(v) for v in first.get(FlightDataType.TYPE_VELOCITY_TOTAL)]
    separations = [e for e in first.getEvents() if events.name_of(e.getType()) == "STAGE_SEPARATION"]
    at_separation = None
    if separations:
        row = nearest(times, float(separations[0].getTime()))
        at_separation = {"time_s": times[row], "altitude_m": altitude[row], "speed_m_s": speed[row]}
    return {
        "configuration_index": CONFIGURATION,
        "booster_separation_event": events.name_of(separation.getSeparationEvent()),
        "booster_separation_delay_s": float(separation.getSeparationDelay()),
        "motors": [motor(m, h) for m, h in mounts],
        "branches": int(data.getBranchCount()),
        "branch_events": branches,
        "first_branch_at_separation": at_separation,
        "first_branch_last_row": {
            "time_s": times[-1],
            "altitude_m": altitude[-1],
            "speed_m_s": speed[-1],
        },
    }


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: first_burnout.py OUTPUT.json")
    output = Path(sys.argv[1])
    logging.disable(logging.CRITICAL)
    events.start()
    import jpype
    from info.openrocket.core.util import BuildProperties
    from java.lang import System

    probes = {probe: flight(probe) for probe in PROBES}
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(
        json.dumps(
            {
                "source": "validation/oracles/openrocket/first_burnout.py",
                "generated": GENERATED,
                "inputs_sha256": {
                    name: hashlib.sha256(Path(module.__file__).read_bytes()).hexdigest()
                    for name, module in [
                        ("first_burnout.py", sys.modules[__name__]),
                        ("events.py", events),
                    ]
                },
                "openrocket": str(BuildProperties.getVersion()),
                "jar_sha256": hashlib.sha256(automatic_radius.JAR.read_bytes()).hexdigest(),
                "example": EXAMPLE,
                "java": str(System.getProperty("java.version")),
                "jpype": jpype.__version__,
                "seed": SEED,
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
