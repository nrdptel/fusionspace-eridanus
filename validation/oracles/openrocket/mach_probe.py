"""A public design that passes Mach 1, for OpenRocket 24.12 to fly (M2.3c11).

M2.3c11 holds HPR Sim's outside thrust and mass histories to a test: a flight given the histories
of its own run, sampled at the times of OpenRocket's own flight of the same design, must come back
within 0.05% of its own apogee, on at least two public designs, one of them passing Mach 1. The
committed public designs both codes fly stay below Mach 0.81 in OpenRocket's flights
(`pods-motors-2`); OpenRocket's own example that passes Mach 1 is read from its jar under the
gitignored `refs/`, so only a machine that fetched the jar can fly it. So this script writes one
that passes it: `pod_probes.py`'s airframe with no pods (a conical nose, a tube 60 mm across with
three fins), its 29 mm mount and H128W swapped for a 38 mm mount 0.5 m long holding an AeroTech
J570W, which OpenRocket's database and HPR Sim's catalog hold as the same curve, a largest
difference of 0 in `crates/hpr-motor/data/openrocket-curves.json`. Every other part, the
material, the ids and the motor configuration are `pods-none`'s. It has no stored simulation, so
OpenRocket flies it in its defaults, as `histories.py` flies the public designs.

Run from the repository root, then `histories.py` as its docstring says:

    refs/venv/bin/python validation/oracles/openrocket/mach_probe.py \\
        validation/fixtures/ork/history-flights

OpenRocket is not run to write it, but `pod_probes` needs the oracle environment (JPype). The
file is plain XML, which OpenRocket and HPR Sim both read.
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import pod_probes  # noqa: E402 - the airframe, its motor configuration and fixed ids

#: AeroTech J570W as OpenRocket 24.12's database holds it, 1,034 N·s.
DIGEST = "14373c65485521035497b1db20e110d3"


def mount():
    """A motor mount 0.5 m long with a 38 mm bore at the bottom of the tube, holding the J570W."""
    return (
        f"<innertube><name>Motor mount</name><id>{pod_probes.uid(3)}</id>"
        '<axialoffset method="bottom">0.0</axialoffset><position type="bottom">0.0</position>'
        "<length>0.5</length><outerradius>0.0195</outerradius><thickness>0.0005</thickness>"
        f"{pod_probes.MATERIAL}<motormount><ignitionevent>automatic</ignitionevent>"
        "<ignitiondelay>0.0</ignitiondelay><overhang>0.0</overhang>"
        f'<motor configid="{pod_probes.CONFIGURATION}"><type>single</type>'
        "<manufacturer>AeroTech</manufacturer><designation>J570W</designation>"
        f"<digest>{DIGEST}</digest><diameter>0.038</diameter><length>0.479</length>"
        "<delay>0.0</delay></motor></motormount></innertube>"
    )


def document():
    """`pods-none`'s document, renamed, with the 38 mm mount and its J570W."""
    text = pod_probes.document("mach-probe", "")
    held = pod_probes.mount(3, "Motor mount")
    if text.count(held) != 1:
        raise RuntimeError("pod_probes.document no longer holds its motor mount as expected")
    return text.replace(held, mount())


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: mach_probe.py DIRECTORY")
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    (out / "mach-probe.ork").write_text(document(), encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
