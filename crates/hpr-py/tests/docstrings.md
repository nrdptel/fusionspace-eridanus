# `fusionspace.hpr`

FusionSpace HPR · Sim, a flight simulator for hobby and high-power rockets.

Four types make a flight, in the order a program needs them: an ``Environment``, a ``Motor``, a
``Rocket`` and a ``Flight``; a ``MonteCarlo`` run flies one many times, its inputs scattered.
Values are in SI units, named in every argument and attribute. The guide's Python page walks
through them: https://hpr.fusionspace.co/python.html

A ``FlightLog`` is a flight log read into one record, by its format's own reader or by the column
mapping or record layout you write for your own logger, as ``hpr analyze`` reads it.

Import it as ``from fusionspace import hpr``.

## `DragTable`

A zero-lift drag coefficient `C_D0` against Mach number, flown in place of HPR Sim's own drag
(`Flight(..., drag_table=...)`), as RocketPy's `power_off_drag` and `power_on_drag` are.

`DragTable(power_off, power_on=None, *, reference_diameter_m=None)`: each curve is a sequence
of `(mach, cd)` rows, a list of pairs or a NumPy array of two columns, with Mach numbers that
increase. The power-on curve is flown while a motor thrusts, and the power-off curve the rest
of the time, or all of it when there is no power-on curve. Between rows the coefficient is
interpolated linearly; past the ends it holds the end values. The coefficients are on the
rocket's reference area, a circle of its largest body diameter, unless
`reference_diameter_m` names another; then they are rescaled by the ratio of the two areas.
Only the drag is replaced: the normal force, center of pressure and damping stay HPR Sim's.

`DragTable.from_csv(power_off, power_on=None, *, reference_diameter_m=None)` reads each curve
from a CSV file of two columns, Mach number and `C_D0`, under an optional header row, as
RocketPy's drag files are.

### `DragTable.cd0`

`C_D0` at `mach`, from the power-on curve when `thrusting` and the table has one, on the
table's own reference diameter.

### `DragTable.from_csv`

A table read from CSV files: `power_off`'s path, and optionally `power_on`'s.

### `DragTable.has_power_on`

Whether the table has a power-on curve.

### `DragTable.reference_diameter_m`

The diameter the coefficients are on, m; `None` for the rocket's own.

## `Environment`

Where a rocket flies: the launch site, the US Standard Atmosphere 1976, and a wind.

`Environment(latitude_deg, longitude_deg, elevation_m, *, wind_speed_m_s=0.0,
wind_from_deg=0.0, gravity="ellipsoidal")`: a launch site at `latitude_deg` and
`longitude_deg` (positive east, so negative in the Americas), `elevation_m` above sea level,
with no wind, or a wind of `wind_speed_m_s` blowing **from** `wind_from_deg`, clockwise from
true north, the same at every height. Or a wind of your own, as a Python function given as
`wind`: called as `wind(height_m)` with a height above sea level, m, it returns the air's
velocity `(east_m_s, north_m_s)`, the way the air moves (a west wind is `(+speed, 0)`). An
exception it raises stops the flight and is raised by `Flight`.

`gravity` says how gravity is found along the flight: `"ellipsoidal"`, the default, is the
full normal gravity vector at the rocket's position; `"vertical"` is its exact size along the
launch site's vertical; and `"vertical_taylor"` is RocketPy's formula, a Taylor series in
height along the vertical, for like-for-like comparisons with RocketPy.

## `Flight`

A flight: `rocket` launched in `environment` from a rail `rail_length_m` long, flown to the
ground as soon as it is made.

`Flight(rocket, environment, rail_length_m, *, inclination_deg=90.0, heading_deg=0.0,
interval_s=None, drag_table=None, drag=None)`: the rail is measured from the rocket's aft end to the rail's top, and leans
`inclination_deg` above the horizon (90 is vertical) toward `heading_deg`, clockwise from true
north. The flight is recorded every `interval_s` seconds, at least 0.001 s, and at every event;
or at every step of the integrator when that is left out. The rocket is copied as the flight
starts, and a flight runs to its end: it can't be interrupted. A `DragTable` given as
`drag_table` is flown in place of HPR Sim's own drag, and so is a Python function given as `drag`:
called as `drag(mach, thrusting)`, with `thrusting` true while a motor burns, it returns the
rocket's zero-lift drag coefficient `C_D0` on the rocket's reference area. An exception a
function raises, the drag's or the environment's wind's, stops the flight, even at a trial
step the integrator would have retried shorter, and is raised here as it was raised.

Heights are the rocket's center of gravity's, above the launch site: it stands on the rail at
the start, so the first height isn't zero. Speeds are relative to the ground.

### `Flight.apogee_m`

The apogee, m above the launch site; `None` if the flight has none.

### `Flight.apogee_time_s`

The time of the apogee after launch, s.

### `Flight.columns`

The recording's column names, each with its unit: `time_s`, `height_above_ground_m`,
`position_east_m`, `airspeed_m_s`, `mach`, `thrust_n`, `mass_kg` and the rest.

### `Flight.envelope_flags`

Where the flight's numbers deserve less trust, each a dictionary: its `flag`
(`"unstable_under_power"`, `"unstable_without_margin"`, `"beyond_validated_range"`,
`"high_angle_of_attack"`, `"outside_core_band"` or `"beyond_envelope"`), the peak that
raised it (`value`, a static margin in calibers, a pitch-moment slope per radian, a Mach
number or an angle of attack in radians, with its `time_s` and `height_above_ground_m`)
and a `message` saying what it means. An empty list for a flight
that raises none.

### `Flight.events`

The flight's events in order, each a dictionary: its `kind` (`"liftoff"`, `"rail_exit"`,
`"burnout"`, `"apogee"`, `"deployment"`, `"ground_hit"` and others), the `index` of the
device or motor it is about (`None` for none), its `time_s` after launch, and the
flight's state then, `sample`, as `hpr_sim::Sample` writes it in JSON (`cg_enu_m` is the
center of gravity east, north and up of the pad, m).

### `Flight.issue_warnings`

The known errors in HPR Sim's drag, stability or flight path this flight meets, each a
dictionary: its `issue` number on GitHub (18, 67, 68, 70, 72, 73 or 222 for drag; 179 or
354, a separated part's, also drag; 64, 87, 120, 121, 172, 325 or 326 for stability; 8,
106, 213 or 219 for the flight's path, whose signs depend on the case), its `kind`
(`"drag"`, `"stability"` or `"flight"`), its `url`, the flight's top Mach number
(`max_mach`, with its `time_s`), the `parts` whose shape meets the issue's condition (their
ids; empty for base drag, 68, and friction, 18, which every rocket has, and for 8, 121, 172,
179, 219 and 354) and a `message` saying which way its numbers lean. An empty list for a
flight that meets none.

### `Flight.keys`

The recording's column names, as `columns`: with `flight[name]`, a flight reads as a
dictionary of arrays.

### `Flight.landing`

Where and how it landed, as a dictionary: `time_s`, `latitude_deg`, `longitude_deg`,
`east_m` and `north_m` of the pad, `distance_m`, `ground_hit_speed_m_s`,
`descent_rate_m_s`, and `body`, the separated body it is about (`None`, the rocket
itself, for a flight with no separation); `None` if it didn't land.

### `Flight.max_mach`

The top Mach number.

### `Flight.max_speed_m_s`

The top speed, m/s.

### `Flight.rail_exit_speed_m_s`

The speed leaving the rail, m/s.

### `Flight.series`

The recording, one NumPy array per column, by name.

### `Flight.summary`

Every metric of the flight, as a dictionary: apogee, peaks, stability margins and
landings, as `hpr_sim::metrics::FlightSummary` writes them in JSON.

### `Flight.to_json`

The flight as JSON text: its result (its events, how it ended, the integrator's work), its
summary and its drag issue warnings, as the Rust `hpr::Flight` writes them.

## `FlightLog`

A flight log, read: a clock, every channel the logger recorded on it in SI units (degrees for
angles), and what the file states about itself, as ``hpr analyze --record`` prints it. Read
one with ``FlightLog.read``.

### `FlightLog.altitude_m`

The primary track, the barometric altitude above the logger's zero (for a mapped table,
the pad), m, on ``time_s``; NaN at a gap.

### `FlightLog.channels`

Every column the file holds, in its order, the primary track's among them: each a dict of
its ``column`` name, the ``quantity`` it measures, its ``unit``, and NumPy arrays of its
``time_s`` and ``values``, NaN at a gap.

### `FlightLog.events`

What the logger says happened and when: each a dict of its ``time_s`` and ``name``.

### `FlightLog.firmware`

The logger's firmware version, as the file states it, or ``None``.

### `FlightLog.flight_number`

The flight's number in the logger's memory, as the file states it, or ``None``.

### `FlightLog.format`

The file format it was read as, such as ``"generic_table"`` for a table read by a mapping.

### `FlightLog.header_mapping`

What the header of the table at ``path`` states, as a column mapping, a dict, to edit and
give back to ``FlightLog.read`` as ``mapping``: set each column's ``quantity`` and
``unit``, and ``primary`` to the column, from 0, that is the barometric altitude above
the pad. ``sheet`` names an ``.xlsx`` workbook's sheet, from 0.

### `FlightLog.logger`

The logger, as the file names it; ``"unknown"`` for one read by a mapping or a layout.

### `FlightLog.notes`

What the reader noticed and worked around, such as a column kept as written.

### `FlightLog.read`

Reads the flight log at ``path``: by its format's own reader, or by a column mapping, a
table's, or a record layout, binary records' or a serial console's capture of them. A
mapping or a layout is a dict, or the path of its JSON file, as the schemas
``schema/cli/analyze-mapping.schema.json`` and ``analyze-layout.schema.json`` describe;
``FlightLog.header_mapping`` gives the mapping a table's header states. ``sheet`` names a
spreadsheet's workbook's sheet (``.xlsx`` or ``.xls``), from 0, to read by its format's
own reader, a mapping or a FIP header; with none, the workbook's one sheet that reads as a
log. ``recording`` names an Entacore AIM raw dump's recording, from 0. ``fip_header`` reads
a Featherweight Raven's FIP export that lost its header line under the header it states, a
dict or a JSON file's path (``schema/cli/analyze-fip-header.schema.json``). Raises
``HprError`` naming the field a mapping, layout or header fails, the choice a file of
several records needs, or why the log doesn't read.

### `FlightLog.readings`

The readings taken from the log on its own, as ``hpr analyze`` takes them: a dict of
``liftoff``, ``apogee``, ``max_speed``, ``max_acceleration``, ``burnout``, ``landing``,
``flight_time``, ``drogue_descent`` and ``main_descent``, each with ``status`` ``"read"``
and its value, the bound its error stays within, the track it came from (``source``),
whether it was measured, derived or clipped (``provenance``) and how it was read
(``method``), or ``"withheld"`` with a ``reason`` and its ``detail``; a top acceleration
also says whether it is a floor (``is_floor``) and why it is clipped, ``"range"`` or
``"flat"`` (``clip``), or ``None``. Beside them, how
they were taken: the track's ``source``, ``sample_interval_s`` and ``step_m``, the running
median's ``median_window_s`` and ``peak_bound_m``, ``pad_altitude_m``, and the axial
``accelerometer`` the top acceleration and burnout came from, or ``None``.

### `FlightLog.serial_number`

The logger's serial number, as the file states it, or ``None``.

### `FlightLog.stated`

What the file states about the flight, such as the logger's own apogee, as a dict.

### `FlightLog.time_s`

The primary track's times, s, from the logger's zero, increasing: empty for a log with no
primary track, such as a table read by a mapping that marks none.

### `FlightLog.to_dict`

The whole record as a dict of lists, gaps as ``None``: the document ``hpr analyze
--record`` prints, without its ``tool``.

## `HprError`

An error from FusionSpace HPR: an input out of its domain, a part out of order, a motor or material that isn't there, or a flight that failed. The message is the library's.

## `MonteCarlo`

A Monte Carlo run: `rocket` flown `runs` times from the same rail in `environment`, each
flight's inputs drawn at random about the nominal flight's, flown as soon as it is made.

`MonteCarlo(rocket, environment, rail_length_m, *, runs=100, seed=0, inclination_deg=90.0,
heading_deg=0.0, drag_table=None, drag=None, mass_sd_fraction=0.0, cg_sd_m=0.0,
drag_sd_fraction=0.0, impulse_sd_fraction=0.0, burn_time_sd_fraction=0.0, delay_sd_s=0.0,
wind_sd_fraction=0.0, wind_from_sd_deg=0.0, inclination_sd_deg=0.0, heading_sd_deg=0.0,
deployment_lag_sd_s=0.0)`: the rocket, the environment and the rail as `Flight` takes them,
and one standard deviation of each input the run scatters, each 0 (not scattered) unless
given, as `hpr mc`'s options are, with the same names as its JSON output: each stage's mass
without motors and each motor's impulse and burn time, the rocket's drag coefficient and the
wind's speed as fractions of their nominal values (0.02 is 2%); each stage's center of mass,
m; each motor's ejection delay and each parachute's lag after its trigger, s; the wind's
direction and the rail's angle and heading, degrees. Each input is drawn from a normal
distribution about its nominal value, independently. The guide's Monte Carlo page says what
each does and where to find numbers for them.

The run is the Rust library's `MonteCarlo::run`, its flights spread over the machine's
threads: a flight's draws depend only on the seed, its index and the inputs, so the same
seed, rocket and options fly the same flights, bit for bit, on one platform, on any number of
threads, and `hpr mc` built the same way (both release builds) flies them too. A flight that fails, its draw impossible (such as a
negative mass) or its flight stopped with an error, is counted (`failed`, `failures`) and
kept in the table, never dropped.

The run reads as a dictionary of NumPy arrays, one a column of `hpr mc --export`'s table, a
value a flight in index order: `run["apogee_m"]`. `columns` lists them: what each flight drew
and what it came to. A number is a `float64` array, NaN where a flight has none (a failed
flight, or a peak its flight didn't reach); `index` is a `uint64` array, and `outcome`,
`failed_at`, `reason`, `termination` and `flags` are arrays of strings, empty where a flight
has none.

A drag function or a wind function is called from the run's threads, one call at a time, and
an exception one raises stops the flights still to fly and is raised here. The calls come in
no set order, so a run repeats bit for bit only with a function whose answer depends on its
arguments alone.

### `MonteCarlo.columns`

The table's column names, in `hpr mc --export`'s order: `index`, `outcome`, `failed_at`
and `reason`; what each flight drew, such as `drag_scale`, `stage_1_dry_mass_scale` and
`motor_1_impulse_scale`; and what it came to, such as `apogee_m`, `landing_distance_m` and
`flags`.

### `MonteCarlo.dispersion`

Each standard deviation the run was given, by its argument's name, 0 for one not
scattered.

### `MonteCarlo.failed`

How many flights failed.

### `MonteCarlo.failures`

The failed flights, grouped by where they failed and why, as `hpr mc`'s JSON lists them:
one dictionary each, with `at` (`"inputs"`, making the flight's inputs from its draw, or
`"flight"`, flying them), `reason`, `count` and `first_index`, the first such flight's
row in the table.

### `MonteCarlo.flown`

How many flights flew: `runs` less `failed`.

### `MonteCarlo.keys`

The column names, as `columns`: with `run[name]`, a run reads as a dictionary of arrays.

### `MonteCarlo.nominal`

The nominal flight's summary, every input at its nominal value, as `Flight.summary` gives
it.

### `MonteCarlo.runs`

How many flights the run flew, failed ones included.

### `MonteCarlo.seed`

The run's seed.

### `MonteCarlo.to_csv`

The table as CSV text, the bytes `hpr mc --export` writes for the same run: a header of
the columns in words with their units in brackets (`apogee [m]` for `run["apogee_m"]`),
then a line a flight, each number in the shortest form that reads back
to the same bits, a missing one an empty cell.

## `Motor`

A solid rocket motor: its thrust curve, masses and size, and its ejection delay.

### `Motor.delay_s`

The ejection delay, s, where it is a number of seconds; `None` for none, or a plugged
motor.

### `Motor.designation`

The designation, such as `"H54"`.

### `Motor.diameter_m`

The case diameter, m.

### `Motor.from_catalog`

The motor in HPR Sim's built-in catalog whose designation or common name is `name`
(`"H54"`, `"168H54-10A"`), ignoring case, spaces and hyphens, with its first bundled
thrust curve; `delay_s` sets its ejection delay. The designation's own delay is not used.

### `Motor.from_eng`

The one motor in a RASP `.eng` file's text.

### `Motor.from_file`

The one motor in a `.eng` or `.rse` file, by its extension.

### `Motor.from_rse`

The one motor in a RockSim `.rse` file's text.

### `Motor.length_m`

The case length, m.

### `Motor.propellant_mass_kg`

The propellant's mass at ignition, kg.

### `Motor.thrust_curve`

The thrust curve's points: two NumPy arrays, the times after ignition, s, and the
thrusts, N.

### `Motor.total_impulse_ns`

The total impulse, N·s.

## `Rocket`

A rocket: built part by part from the nose back, or read from a design file.

`Rocket(name, diameter_m)`: a rocket called `name`, `diameter_m` across outside, its first
tube's diameter and its nose's.

Parts go on in order: the nose, then tubes and transitions; fins, a motor tube and masses go
on or in the tube before them. Every part names its material by a built-in id
(`hpr.materials()`). Each method returns the rocket, so calls can be chained.

### `Rocket.add_fins`

Adds `count` trapezoidal fins `thickness_m` thick to the tube before them: a root chord,
tip chord and span, and the sweep, how far aft of the root's leading edge the tip's is.
Every argument after `count` is named, so two lengths can't swap unseen.
`cross_section` is `"square"`, the default, `"rounded"` or `"airfoil"`. They sit flush
with the tube's aft end unless `position` (`"top"`, `"middle"`, `"bottom"` or `"after"`)
and `offset_m`, aft of it, place them.

### `Rocket.add_mass`

Adds a mass of `mass_kg` in the tube before it, placed by `position` (`"top"`, the
default, `"middle"`, `"bottom"` or `"after"`) and `offset_m` aft of there: a point, or
packed in a cylinder `packed_length_m` long and `packed_diameter_m` across.

### `Rocket.add_motor_tube`

Adds a motor tube `length_m` long with an `inner_diameter_m` bore and a wall `wall_m`
thick, in the tube before it: flush with that tube's aft end unless `position` and
`offset_m` place it. The motor's nozzle sticks out `overhang_m` past its aft end.

### `Rocket.add_nose`

Adds the nose cone, which goes first. `shape` is `"conical"`, `"ogive"` (`parameter`, the
ogive's radius ratio, 1 for a tangent ogive, by default), `"elliptical"`,
`"power_series"` (`parameter`, its exponent), `"parabolic_series"` (`parameter`, `K`) or
`"haack"` (`parameter`, `C`: 0, the default, is the von Kármán). A `wall_m` makes it
hollow, and leaving it out solid; a `shoulder_length_m` and `shoulder_wall_m` add a
shoulder, capped at its aft end with `capped_shoulder`.

### `Rocket.add_parachute`

Adds a parachute called `name`: a canopy `diameter_m` across (its `canopy` type's drag
coefficient, `"flat_circular"` by default, unless `drag_coefficient` gives one), or a
drag area `cd_s_m2`, m², as RocketPy's `cd_s`. It opens when `trigger` fires:
`"apogee"`; `"altitude"`, on the way down past `altitude_m` above the pad; `"time"`, at
`time_s` after launch; or `"motor_delay"`, at the ejection delay of the motor numbered
`motor` (0, the first, by default), which needs a delay in seconds. It is fully open
`lag_s` seconds later. With `released_by`, the number of another parachute (0 for the
first added, 1 for the second, and so on), it is cut away once that one is fully open, as
a drogue is when the main opens; RocketPy flies only its last parachute to open, which is
the same. An argument that doesn't apply to the choices made is refused, and a
`released_by` that names no other parachute is refused when the rocket flies.

### `Rocket.add_transition`

Adds a transition `length_m` long from the part before it to `aft_diameter_m`: a
shoulder or a boattail. It is conical unless `shape` names one of the nose's shapes, and
hollow with `wall_m` unless `solid`.

### `Rocket.add_tube`

Adds a body tube `length_m` long with a wall `wall_m` thick, behind the part before it:
the rocket's diameter across, or `diameter_m`.

### `Rocket.configuration`

The motor configuration flown, by id; `None` before a motor is set.

### `Rocket.design_json`

The design as JSON text: the `hpr_design::Rocket` a rocket's JSON file holds.

### `Rocket.from_file`

A rocket read from a design file, flown in its motor configuration `configuration`, by
id: an HPR design (`.hpr` or `.hprz`), an OpenRocket `.ork` file, or a rocket's JSON
(`.json`, an `hpr_design::Rocket`). Left out, the configuration is the design's only one.
Parts can't be added to it; parachutes can.

With `recovery=True`, the file's recovery devices fly as `hpr sim` flies them
(`hpr::ork::recovery`): each opens fully at its event, with the file's drag coefficient
or OpenRocket's own, and once one opens the rocket descends as a point under the open
devices' drag alone. They come before any added with `add_parachute`, so the file's first
device is parachute 0. A device that never opens in the configuration is named in
`notes`, and a file whose devices HPR Sim can't fly is refused with why. Left out, the file's
devices aren't flown.

### `Rocket.margin`

The whole stability picture `time_s` after ignition at `mach`: the margin, the center of
pressure (`cp_station_m`, meters aft of the nose tip) and the slopes they come from, as a
dictionary.

### `Rocket.mass_properties`

The mass, center of gravity and inertia `time_s` after ignition: a dictionary of
`mass_kg`, `cg_m` (x, y, z in the body frame, where a point `s` meters aft of the nose
tip is at `z = -s`) and `inertia_kg_m2` (the 3×3 tensor about the center of gravity, a
list of its rows).

### `Rocket.name`

The rocket's name.

### `Rocket.notes`

What reading the design file said, one line each: the `.ork` reader's warnings, an older
format version migrated, an airframe not flown as written, the recovery devices the file
holds (not flown, unless `from_file`'s `recovery=True` flies them, and then any that never
opens) and the stages that fly as one stack. Empty for a rocket built part by part.

### `Rocket.set_motor`

Puts the motor in the motor tube.

### `Rocket.static_margin_cal`

The static stability margin, calibers, `time_s` after ignition at `mach`, with the air
along the axis, in the rocket's weakest plane (a fin set of one or two fins makes some
planes weaker than others); `None` where the rocket has no normal force to find a center of
pressure from.

## `materials`

The built-in materials' ids, the names `Rocket`'s parts take: `"abs"`, `"kraft_phenolic"`,
`"birch_plywood"` and the rest, each with its density's source in the Rust documentation of
`hpr_design::materials`.
