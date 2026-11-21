"""Two upstream corrections to RocketPy 1.13.0's equations of motion, which the whole-flight oracle
flies (M2.1d3, ADR-026, issue #50).

RocketPy's default 6-DOF equations, `Flight.u_dot_generalized`, take the centre of dry mass (CDM)
as their reference point and need the vectors *from* it to the instantaneous centre of mass
(`r_CM`) and to the nozzle exit (`r_NOZ`). They read `Rocket.com_to_cdm_function` and
`Rocket.nozzle_to_cdm`, which point the other way, *to* the CDM (`rocket.py:996-1025`, whose note
says `com_to_cdm_function + center_of_mass == center_of_dry_mass_position`, and `:959-972`). So
while the motor burns, RocketPy takes the moments about a point as far forward of the CDM as the
true centre of mass is behind it. For Juno III at its rail exit that point is 1.315 m from the nose
tip where the centre of mass is 1.639 m, so the margin it flies there, to its centre of pressure at
2.072 m, is 1.75 times its own `static_margin`. After burnout `r_CM` is zero and `r_NOZ` enters
only through the mass flow, so the error ends with the burn.

Both are on RocketPy's record:

- An outside contributor reported the sign in issue #1186 (2026-08-25) and proposed the fix
  in PR #1196 (head 927e771e1a7faa2915d1f96ff48a70c544db13b4, 2026-09-16, open and not yet
  reviewed when it was adopted here; merged into `develop` on 2026-11-16 as `aee512f8`, in no
  release): three edits to `u_dot_generalized` that negate `r_CM` and its derivatives, negate
  `r_NOZ`, and flip the sign of the `r_CM ^ w_dot` term in `v_dot`, which was written for the
  reversed vector. RocketPy's `develop` branch agreed on the convention before that merge: its
  tip-off phase as PR #920 added it (merged 2026-09-14) said the generalized equations "store
  r_CM / r_NOZ as (point -> CDM) vectors, i.e. the negative of the true-frame position", and
  negated them for its own use. PR #1217's rewrite of the phase no longer carries that comment.
- PR #1188 (merged into `develop` 2026-09-09, not yet released) corrects the nozzle gyration
  tensor's parallel-axis term from `0.25 * nozzle_to_cdm**2` to `nozzle_to_cdm**2`
  (`rocket.py:984-985`), so the jet damping about the CDM uses the whole lever.

This module applies exactly those changes and nothing else. `CorrectedFlight` recompiles
RocketPy's own `u_dot_generalized` source with #1196's three substitutions. On a source without
them each must match exactly once, so an upgrade that moves the code stops the script instead of
flying something else. A source that already holds all three corrected lines once and none of
the released ones (`upstream_carries_1196`: `develop` since #1196's merge) is flown as it is.
`correct_nozzle_gyration` sets the tensor as #1188's `evaluate_nozzle_gyration_tensor` does.

Checked before adopting them (ADR-026):

- RocketPy against itself, which `wind_response.py` prints for every case: at the rail exit, where
  the rotation rate is zero so no damping term acts, RocketPy's angular acceleration as released
  equals the moment about the mirrored point over the inertia, not the moment about its own
  `center_of_mass` (Juno III: 0.3608 against 0.2067 rad/s^2).
- Against hpr, with a local probe that is not committed: at RocketPy's own states along Juno III's
  windy flight, the corrected equations give the angular acceleration hpr's do to 0.1 to 5%, and
  the centre-of-dry-mass acceleration to 0.01 m/s^2, once hpr's normal force is made linear like
  RocketPy's; as released they give 1.75 times hpr's at the rail exit.

RocketPy is MIT-licensed (`THIRD-PARTY-NOTICES.md`); the substituted lines are PR #1196's.
"""

import inspect
import linecache
import textwrap

import rocketpy.simulation.flight as rocketpy_flight
from rocketpy import Flight
from rocketpy.mathutils.vector_matrix import Matrix

# What the fixture records about the corrections, in order.
CORRECTIONS = [
    {
        "upstream": "https://github.com/RocketPy-Team/RocketPy/pull/1188",
        "state": "merged into develop 2026-09-09, not in a release",
        "what": "the nozzle gyration tensor's parallel-axis term is nozzle_to_cdm**2, not "
                "0.25 * nozzle_to_cdm**2 (rocket.py:984-985), so the jet damping about the "
                "centre of dry mass uses the whole lever",
    },
    {
        "upstream": "https://github.com/RocketPy-Team/RocketPy/pull/1196",
        "issue": "https://github.com/RocketPy-Team/RocketPy/issues/1186",
        "state": "merged into develop 2026-11-16 (aee512f8), not in a release; adopted at head "
                 "927e771e1a7faa2915d1f96ff48a70c544db13b4 (2026-09-16)",
        "what": "u_dot_generalized negates com_to_cdm_function and nozzle_to_cdm where it reads "
                "them and flips the sign of r_CM ^ w_dot in v_dot (flight.py:2509-2515, :2695): "
                "both attributes point to the centre of dry mass, the equations need them from "
                "it, so during the burn the moments were taken about a point as far forward of "
                "the dry centre of mass as the centre of mass is behind it",
    },
]

# PR #1196's edits to RocketPy 1.13.0's `Flight.u_dot_generalized`, as (released, corrected).
SUBSTITUTIONS = [
    (
        "        r_CM_z = self.rocket.com_to_cdm_function\n"
        "        r_CM_t = r_CM_z.get_value_opt(t)\n"
        "        r_CM = Vector([0, 0, r_CM_t])\n"
        "        r_CM_dot = Vector([0, 0, r_CM_z.differentiate_complex_step(t)])\n"
        "        r_CM_ddot = Vector([0, 0, r_CM_z.differentiate(t, order=2)])\n",
        "        com_to_cdm = self.rocket.com_to_cdm_function\n"
        "        r_CM = Vector([0, 0, -com_to_cdm.get_value_opt(t)])\n"
        "        r_CM_dot = Vector([0, 0, -com_to_cdm.differentiate_complex_step(t)])\n"
        "        r_CM_ddot = Vector([0, 0, -com_to_cdm.differentiate(t, order=2)])\n",
    ),
    (
        "        r_NOZ = Vector([0, 0, self.rocket.nozzle_to_cdm])\n",
        "        r_NOZ = Vector([0, 0, -self.rocket.nozzle_to_cdm])\n",
    ),
    (
        "        v_dot = K @ (T20 / total_mass - (r_CM ^ w_dot)) - 2 * (w_earth ^ v)\n",
        "        v_dot = K @ (T20 / total_mass + (r_CM ^ w_dot)) - 2 * (w_earth ^ v)\n",
    ),
]


def upstream_carries_1196(source):
    """Whether this `u_dot_generalized` source already has PR #1196's three edits: each corrected
    form once and no released form. RocketPy merged the pull request into `develop` on
    2026-11-16 (`aee512f8`), so a checkout past that needs no substitution. A source with some
    of the edits and not others is neither, and `corrected_u_dot_generalized` stops on it."""
    return all(
        source.count(corrected) == 1 and source.count(released) == 0
        for released, corrected in SUBSTITUTIONS
    )


def substituted(source):
    """`source` with PR #1196's three substitutions, each made exactly once; stops the script if
    any released form is not there exactly once, or if the result does not then carry the three
    edits as `upstream_carries_1196` asks (a corrected line already beside a released one)."""
    for released, corrected in SUBSTITUTIONS:
        found = source.count(released)
        if found != 1:
            raise SystemExit(
                f"corrections.py: RocketPy's u_dot_generalized holds {found} copies of "
                f"{released!r}, not one; it has changed, so PR #1196's edits no longer apply as "
                "written"
            )
        source = source.replace(released, corrected)
    if not upstream_carries_1196(source):
        raise SystemExit(
            "corrections.py: RocketPy's u_dot_generalized already held some of PR #1196's "
            "corrected lines beside the released ones, so its edits no longer apply as written"
        )
    return source


FILENAME = "<u_dot_generalized with PR #1196>"


def compiled(source, filename=FILENAME):
    """`source`, one `Flight` method's text, compiled inside a class named `Flight` in
    `rocketpy.simulation.flight`'s globals, so its private names (`self.__post_processed_variables`)
    mangle as they do upstream. Its lines go into `linecache` under `filename`, so a traceback from
    inside it shows them."""
    wrapped = "class Flight:\n" + textwrap.indent(textwrap.dedent(source), "    ")
    linecache.cache[filename] = (len(wrapped), None, wrapped.splitlines(True), filename)
    namespace = {}
    exec(compile(wrapped, filename, "exec"), vars(rocketpy_flight), namespace)
    return namespace["Flight"].u_dot_generalized


def corrected_u_dot_generalized():
    """RocketPy's own `u_dot_generalized`, recompiled (`compiled`) with PR #1196's substitutions
    (`substituted`), or as it is where the source already carries them (`upstream_carries_1196`).
    """
    source = inspect.getsource(Flight.u_dot_generalized)
    if upstream_carries_1196(source):
        return Flight.u_dot_generalized
    return compiled(substituted(source))


class CorrectedFlight(Flight):
    """RocketPy 1.13.0's `Flight` with PR #1196's `u_dot_generalized`.

    `Flight.__init__` binds `self.u_dot_generalized` for the default `equations_of_motion`, so the
    corrected method is the one every phase from the rail exit to apogee integrates, and the one
    the post-processing re-evaluates.
    """

    u_dot_generalized = corrected_u_dot_generalized()


def correct_nozzle_gyration(rocket):
    """Sets `rocket.nozzle_gyration_tensor` as PR #1188's `evaluate_nozzle_gyration_tensor` does:
    the exit disc's second moment per unit area about the centre of dry mass, `r^2/4 + l^2`
    across and `r^2/2` along the axis. Call it after `add_motor`, which evaluates the tensor."""
    radius = rocket.motor.nozzle_radius
    axial = 0.5 * radius**2
    lateral = 0.5 * axial + rocket.nozzle_to_cdm**2
    rocket.nozzle_gyration_tensor = Matrix(
        [[lateral, 0, 0], [0, lateral, 0], [0, 0, axial]]
    )


def check():
    """`upstream_carries_1196` and `substituted` on sources built from the installed release's
    `u_dot_generalized`: the release itself (corrected here), the release with all three edits
    (flown as it is, as on `develop` since #1196's merge), that source with one edit undone,
    doubled, or kept beside its released form, and the release with one released form doubled or
    with one corrected form beside it (neither, so the correction stops). Then that the function
    `CorrectedFlight` flies is the release compiled with all three edits. Returns the failures and
    the number of sources. `tip_off.py` checks the pinned `develop` commit's own source with
    `upstream_carries_1196`."""
    released_source = inspect.getsource(Flight.u_dot_generalized)
    if upstream_carries_1196(released_source):
        return ["the installed RocketPy already carries #1196, so there is no release source to "
                "build the check's sources from: build them from 1.13.0's text, or retire the "
                "substitution (ADR-026)"], 0
    merged_source = substituted(released_source)
    cases = [("the released source", released_source, False),
             ("the released source with all three edits", merged_source, True)]
    for index, (released, corrected) in enumerate(SUBSTITUTIONS, start=1):
        cases.append((f"edit {index} undone", merged_source.replace(corrected, released), False))
        cases.append((f"edit {index} doubled", merged_source.replace(corrected, 2 * corrected),
                      False))
        cases.append((f"edit {index} beside its released form",
                      merged_source.replace(corrected, corrected + released), False))
        cases.append((f"released form {index} doubled",
                      released_source.replace(released, 2 * released), False))
        cases.append((f"released form {index} beside its edit",
                      released_source.replace(released, released + corrected), False))
    failures = [
        f"{name}: upstream_carries_1196 gave {not expected}, not {expected}"
        for name, source, expected in cases
        if upstream_carries_1196(source) != expected
    ]
    # Code objects compare by their bytecode, constants, names and lines, not their file name,
    # so the check compiles under names of its own and leaves the flown function's lines in
    # `linecache` as they were.
    flown = CorrectedFlight.u_dot_generalized.__code__
    if (flown != compiled(merged_source, "<check: all three edits>").__code__
            or flown == compiled(released_source, "<check: the release>").__code__):
        failures.append("CorrectedFlight does not fly the release compiled with all three edits")
    for name, source, _ in cases[2:]:
        try:
            substituted(source)
        except SystemExit:
            continue
        failures.append(f"{name}: substituted corrected it instead of stopping")
    return failures, len(cases)


if __name__ == "__main__":
    import sys

    if sys.argv[1:] != ["--check"]:
        raise SystemExit("usage: corrections.py --check")
    found, sources = check()
    for failure in found:
        print(f"corrections.py --check: {failure}", file=sys.stderr)
    if found:
        raise SystemExit(1)
    print(f"corrections.py --check: {sources} sources classified as expected")
