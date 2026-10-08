"""OpenRocket 24.12's flights of the private collection's tier-A flights, in their day's weather.

M2.3c1 (ADR-184) compares hpr and OpenRocket with the logged apogees of the tier-A flights of
`hpr-sim-fixtures` (ADR-151) whose design is a `.ork`. `cargo xtask fixture-flights conditions`
writes `corpus-out/hpr-sim-fixtures/conditions.json`: for each flight the design, the motor
configuration hpr chose for it, the site, the rail, and the weather of its hour from ERA5 as
OpenRocket can take it. This script flies each in OpenRocket 24.12 with:

- the launch site's latitude, longitude and altitude, the rod's length, angle from vertical and
  direction, and not into the wind;
- the atmosphere OpenRocket offers: the standard one through the pad's temperature and pressure
  (`setISAAtmosphere(False)`, `setLaunchTemperature`, `setLaunchPressure`);
- its multi-level wind (`WindModelType.MULTI_LEVEL`, heights above ground), one level every 50 m
  from the pad to 25 km, each with no turbulence, so the flight is the mean wind's;
- every recovery device set never to deploy, as hpr flies it, so the apogee is the climb's.

and records the largest altitude (OpenRocket's altitude is 0 at launch), the time to it and the
largest Mach number, or why OpenRocket refused or aborted the flight. Before the flights a probe
checks the multi-level wind against the average-wind model whose direction convention
`conditions.py` measured (where the wind blows from, clockwise from north, radians): the simple
example flown in a 5 m/s wind from the east, both ways, must land in the same place.

OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The class and
method names used are the public API that `javap` prints for the jar. The record stays under the
gitignored `corpus-out/`: the flights are private. Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/fixture_flights.py
"""

import hashlib
import json
import logging
import math
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "rocketserializer"))

import automatic_radius  # noqa: E402 - the jar's path
import events  # noqa: E402 - the JVM start with the motor database loaded
import geometry  # noqa: E402 - the same file reading and comment retry as the cross-check

CONDITIONS = Path("corpus-out/hpr-sim-fixtures/conditions.json")
RECORD = Path("corpus-out/hpr-sim-fixtures/openrocket-flights.json")
SEED = 1


def relative(path):
    """A script's path from the repository's root, with forward slashes."""
    return Path(path).resolve().relative_to(Path(__file__).resolve().parents[3]).as_posix()


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def finite(value):
    value = float(value)
    return value if math.isfinite(value) else None


def never_deploy(document, configuration):
    """Sets every recovery device of `document` never to deploy in `configuration`."""
    from info.openrocket.core.rocketcomponent import DeploymentConfiguration, RecoveryDevice

    for device in events.components(document.getRocket(), RecoveryDevice):
        held = device.getDeploymentConfigurations().get(configuration)
        held.setDeployEvent(DeploymentConfiguration.DeployEvent.NEVER)


def set_conditions(options, flight):
    """The flight's site, rail and weather, on OpenRocket's options."""
    from info.openrocket.core.models.wind import WindModel, WindModelType
    from info.openrocket.core.util import GeodeticComputationStrategy

    # The stored simulation's method varies by design; hpr flies the WGS 84 ellipsoid.
    options.setGeodeticComputation(GeodeticComputationStrategy.WGS84)
    options.setLaunchAltitude(flight["launch_altitude_m"])
    options.setLaunchLatitude(flight["launch_latitude_deg"])
    options.setLaunchLongitude(flight["launch_longitude_deg"])
    options.setLaunchRodLength(flight["rod_length_m"])
    options.setLaunchRodAngle(flight["rod_angle_rad"])
    options.setLaunchRodDirection(flight["rod_direction_rad"])
    options.setLaunchIntoWind(False)
    options.setISAAtmosphere(False)
    weather = flight["weather"]
    options.setLaunchTemperature(weather["pad_temperature_k"])
    options.setLaunchPressure(weather["pad_pressure_pa"])
    options.setWindModelType(WindModelType.MULTI_LEVEL)
    wind = options.getMultiLevelWindModel()
    wind.setAltitudeReference(WindModel.AltitudeReference.AGL)
    wind.clearLevels()
    for height, speed, direction in weather["wind_agl_m_speed_m_s_from_rad"]:
        wind.addWindLevel(float(height), float(speed), float(direction), 0.0)
    levels = list(wind.getLevels())
    if len(levels) != len(weather["wind_agl_m_speed_m_s_from_rad"]):
        raise RuntimeError("OpenRocket kept another number of wind levels than it was given")
    for level, (height, speed, direction) in zip(levels, weather["wind_agl_m_speed_m_s_from_rad"]):
        held = (float(level.getAltitude()), float(level.getSpeed()), float(level.getDirection()))
        if any(abs(a - b) > 1e-9 * max(1.0, abs(b)) for a, b in zip(held, (height, speed, direction))):
            raise RuntimeError(f"a wind level reads back as {held}, not {(height, speed, direction)}")
        if float(level.getStandardDeviation()) != 0.0:
            raise RuntimeError("a wind level has turbulence")
    options.setRandomSeed(SEED)


def fly(document, configuration, base, flight):
    """One configuration flown, nothing deployed: the climb's largest altitude, or why not."""
    from info.openrocket.core.document import Simulation
    from info.openrocket.core.simulation import FlightDataType

    simulation = Simulation(document, document.getRocket())
    if base is not None:
        simulation.copySimulationOptionsFrom(base.getOptions())
    simulation.setFlightConfigurationId(configuration)
    options = simulation.getOptions()
    if flight is None:
        return simulation, options
    set_conditions(options, flight)
    never_deploy(document, configuration)
    record = {"geodetic": str(options.getGeodeticComputation().name())}
    try:
        simulation.simulate()
    except Exception as error:  # noqa: BLE001 - a refusal is the measurement
        record["refused"] = "OpenRocket refused to fly it"
        record["detail"] = geometry.first_line(error)
        return record
    data = simulation.getSimulatedData()
    branch = data.getBranch(0)
    altitude = [float(x) for x in branch.get(FlightDataType.TYPE_ALTITUDE)]
    times = [float(x) for x in branch.get(FlightDataType.TYPE_TIME)]
    mach = [float(x) for x in branch.get(FlightDataType.TYPE_MACH_NUMBER)]
    aborts = [
        str(event.getData())
        for event in branch.getEvents()
        if events.name_of(event.getType()) == "SIM_ABORT"
    ]
    finite_altitudes = [a for a in altitude if math.isfinite(a)]
    if not finite_altitudes:
        record["refused"] = "OpenRocket's flight has no altitude"
        return record
    highest = max(finite_altitudes)
    record.update(
        {
            "apogee_m": highest,
            "time_to_apogee_s": times[altitude.index(highest)],
            "max_mach": max((m for m in mach if math.isfinite(m)), default=None),
            "summary_max_altitude_m": finite(data.getMaxAltitude()),
        }
    )
    if aborts:
        # An abort before the apogee leaves no apogee to compare.
        apogee_events = [
            float(e.getTime()) for e in branch.getEvents() if events.name_of(e.getType()) == "APOGEE"
        ]
        if not apogee_events:
            record["refused"] = "OpenRocket aborted the flight before apogee"
            record["detail"] = aborts[0]
            del record["apogee_m"]
    return record


def wind_probe():
    """The simple example in a 5 m/s wind from the east, by the average model and by the
    multi-level one with the same wind at every level: both must land in the same place."""
    from info.openrocket.core.models.wind import WindModelType
    from info.openrocket.core.simulation import FlightDataType
    from info.openrocket.core.util import GeodeticComputationStrategy

    landed = {}
    for model in ["average", "multi-level"]:
        document = events.example(events.SINGLE)
        configuration = document.getRocket().getIds()[0]
        base = list(document.getSimulations())[0]
        simulation, options = fly(document, configuration, base, None)
        options.setLaunchIntoWind(False)
        options.setLaunchRodAngle(0.0)
        options.setRandomSeed(SEED)
        # As `set_conditions` sets it, so the wind model is the only difference.
        options.setGeodeticComputation(GeodeticComputationStrategy.WGS84)
        if model == "average":
            options.setWindModelType(WindModelType.AVERAGE)
            options.setWindSpeedAverage(5.0)
            options.setWindSpeedDeviation(0.0)
            options.setWindTurbulenceIntensity(0.0)
            options.setWindDirection(math.pi / 2)
        else:
            set_conditions(
                options,
                {
                    "launch_altitude_m": float(options.getLaunchAltitude()),
                    "launch_latitude_deg": float(options.getLaunchLatitude()),
                    "launch_longitude_deg": float(options.getLaunchLongitude()),
                    "rod_length_m": float(options.getLaunchRodLength()),
                    "rod_angle_rad": 0.0,
                    "rod_direction_rad": float(options.getLaunchRodDirection()),
                    "weather": {
                        "pad_temperature_k": float(options.getLaunchTemperature()),
                        "pad_pressure_pa": float(options.getLaunchPressure()),
                        "wind_agl_m_speed_m_s_from_rad": [
                            [h, 5.0, math.pi / 2] for h in range(0, 3001, 50)
                        ],
                    },
                },
            )
            options.setISAAtmosphere(True)
        simulation.simulate()
        branch = simulation.getSimulatedData().getBranch(0)
        east = [float(x) for x in branch.get(FlightDataType.TYPE_POSITION_X)]
        north = [float(x) for x in branch.get(FlightDataType.TYPE_POSITION_Y)]
        landed[model] = (east[-1], north[-1])
    (e1, n1), (e2, n2) = landed["average"], landed["multi-level"]
    if not (e1 < -1.0 and abs(e1 - e2) < 1e-6 * abs(e1) and abs(n1 - n2) < 1e-6 * abs(e1)):
        raise RuntimeError(f"the multi-level wind lands elsewhere: {landed}")
    return {"wind_from_east_5_m_s_landed_east_m": [round(e1, 4), round(e2, 4)]}


def main():
    logging.disable(logging.CRITICAL)
    conditions = json.loads(CONDITIONS.read_text(encoding="utf-8"))
    events.start()
    import jpype
    from info.openrocket.core.util import BuildProperties
    from java.lang import System

    probe = wind_probe()
    flights = []
    with tempfile.TemporaryDirectory() as scratch:
        for flight in conditions["flights"]:
            entry = {"flight_id": flight["flight_id"]}
            try:
                text = geometry.document_text(flight["design"])
                document, _ = geometry.opened(text, scratch)
            except Exception as error:  # noqa: BLE001 - a refusal is the measurement
                entry["refused"] = "OpenRocket refused to open the design"
                entry["detail"] = geometry.first_line(error)
                flights.append(entry)
                continue
            rocket = document.getRocket()
            wanted = (flight["configuration"] or "").lower()
            every = list(rocket.getIds())
            ids = [i for i in every if rocket.getFlightConfiguration(i).hasMotors()]
            found = [i for i in ids if str(i.toString()).lower() == wanted]
            if not found and any(str(i.toString()).lower() == wanted for i in every):
                entry["refused"] = "OpenRocket loads no motor into that configuration"
                flights.append(entry)
                continue
            if not found and len(ids) == 1 and flight["configuration"] is not None:
                # OpenRocket gives a configuration whose id is not a UUID a new one.
                found, entry["configuration_renamed"] = ids, True
            if not found:
                entry["refused"] = "no configuration of that id in OpenRocket's reading"
                flights.append(entry)
                continue
            stored = list(document.getSimulations())
            try:
                entry.update(fly(document, found[0], stored[0] if stored else None, flight))
            except Exception as error:  # noqa: BLE001 - a driver fault, recorded
                entry["refused"] = "the driver failed"
                entry["detail"] = geometry.first_line(error)
            flights.append(entry)

    RECORD.parent.mkdir(parents=True, exist_ok=True)
    RECORD.write_text(
        json.dumps(
            {
                "source": "validation/oracles/openrocket/fixture_flights.py",
                # Paths from the repository's root, which `cargo xtask fixture-flights`
                # hashes again before it reads the record.
                "inputs_sha256": {
                    relative(module.__file__): sha256(module.__file__)
                    for module in (sys.modules[__name__], automatic_radius, events, geometry)
                },
                "conditions_sha256": sha256(CONDITIONS),
                "openrocket": str(BuildProperties.getVersion()),
                "jar_sha256": sha256(automatic_radius.JAR),
                "java": str(System.getProperty("java.version")),
                "jpype": jpype.__version__,
                "seed": SEED,
                "wind_probe": probe,
                "flights": flights,
            },
            indent=1,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    refused = sum(1 for f in flights if "refused" in f)
    print(f"{len(flights)} flights, {refused} not flown", file=sys.stderr)


if __name__ == "__main__":
    main()
