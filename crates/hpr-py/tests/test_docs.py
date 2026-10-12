"""The docs' Python, run: each block in order, each printing what its page says it prints.

`docs/python.md` shows each ```python block's output in the ```text block after it. Other pages
mark theirs: a ```python block and the ```text block of its output between
`<!-- python: example -->` and `<!-- python: end -->`, and every marked block on every page under
`docs/` is run here. The blocks of a page run in one namespace, from the repository's root as the
pages' paths assume, and each output is held to the page's text, so no page can go stale.

With `HPR_WRITE_DOCS=1` in the environment the tests write each output into its page instead of
failing on it (`HPR_WRITE_DOCS=1 scripts/python-tests.sh 3.13`); read the diff before committing
it. The guides check (`cargo xtask guides`) lets a number stand in a marked block for this reason.
"""

import contextlib
import io
import os
import re

BLOCK = re.compile(r"```(python|text)\n(.*?)```", re.DOTALL)
PYTHON_EXAMPLE = "<!-- python: example -->"
PYTHON_END = "<!-- python: end -->"
MARKED = re.compile(re.escape(PYTHON_EXAMPLE) + r"\n(.*?)" + re.escape(PYTHON_END), re.DOTALL)
WRITE = os.environ.get("HPR_WRITE_DOCS") == "1"
TOUR = "docs/simulator-tour.md"


def unmarked_blocks(page):
    """Each python block of `page` with the span of the text block after it, if one follows."""
    found = list(BLOCK.finditer(page))
    pairs = []
    for index, block in enumerate(found):
        if block.group(1) == "python":
            after = found[index + 1] if index + 1 < len(found) else None
            pairs.append((block.group(2), after.span(2) if after and after.group(1) == "text" else None))
    return pairs


def marked_blocks(name, page):
    """Each marked python block of `page` with the span of its output's text block. A marker
    that doesn't pair, or a marked region holding anything but one python block and one text
    block, fails: the guides check trusts every number between the markers to be run here."""
    pairs = []
    for region in MARKED.finditer(page):
        inside = list(BLOCK.finditer(page, region.start(1), region.end(1)))
        kinds = [block.group(1) for block in inside]
        assert kinds == ["python", "text"], f"{name}: a marked region holds {kinds}"
        assert page.count("```", region.start(1), region.end(1)) == 4, f"{name}: a stray fence"
        pairs.append((inside[0].group(2), inside[1].span(2)))
    assert page.count(PYTHON_EXAMPLE) == page.count(PYTHON_END) == len(pairs), (
        f"{name}: a python marker without its pair"
    )
    return pairs


def run_page(repo, monkeypatch, name, pairs):
    """Runs a page's blocks in one namespace and holds each output to the page, or with
    `HPR_WRITE_DOCS=1` writes it there. Returns the namespace."""
    path = repo / name
    page = path.read_text(encoding="utf-8")
    monkeypatch.chdir(repo)
    namespace = {}
    outputs = []
    for code, span in pairs:
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            exec(compile(code, name, "exec"), namespace)
        assert span is not None, f"{name}: a python block with no output shown:\n{code}"
        outputs.append((span, out.getvalue()))
    written = page
    for (start, end), output in reversed(outputs):
        written = written[:start] + output + written[end:]
    if WRITE:
        if written != page:
            with open(path, "w", encoding="utf-8", newline="\n") as file:
                file.write(written)
    else:
        for (start, end), output in outputs:
            assert page[start:end] == output, f"{name}: stale output (HPR_WRITE_DOCS=1 rewrites it)"
    return namespace


def test_the_python_page_prints_what_it_says(repo, monkeypatch):
    page = (repo / "docs/python.md").read_text(encoding="utf-8")
    pairs = unmarked_blocks(page)
    assert len(pairs) == 7, "the page's python blocks"
    run_page(repo, monkeypatch, "docs/python.md", pairs)


def test_every_marked_example_prints_what_its_page_says(repo, monkeypatch):
    pages = sorted(path for path in (repo / "docs").rglob("*.md") if PYTHON_EXAMPLE in path.read_text(encoding="utf-8"))
    names = [path.relative_to(repo).as_posix() for path in pages]
    assert TOUR in names, "the tour's python"
    for name in names:
        pairs = marked_blocks(name, (repo / name).read_text(encoding="utf-8"))
        run_page(repo, monkeypatch, name, pairs)


def cli_block(page, command):
    """The text block `cargo xtask cli` regenerates for `command` on `page`."""
    marker = f"<!-- cli: example `{command}` -->"
    assert page.count(marker) == 1, f"the page's example of `{command}`"
    block = BLOCK.search(page, page.index(marker))
    assert block and block.group(1) == "text"
    return block.group(2)


def labelled(block, label, holding=""):
    """The one line of `block` that starts with `label` and then two spaces, as `hpr sim` aligns
    its summary, and holds `holding`: the table of events has an apogee line too."""
    lines = block.splitlines()
    (line,) = [line for line in lines if re.match(re.escape(label) + r"  ", line) and holding in line]
    return line


def test_the_tours_python_flies_what_its_command_line_printed(repo, monkeypatch):
    """The tour says its Python's numbers are the ones `hpr sim` printed above them: the same
    rocket, the same launch, the same code. This holds each to the command's regenerated block."""
    page = (repo / TOUR).read_text(encoding="utf-8")
    namespace = run_page(repo, monkeypatch, TOUR, marked_blocks(TOUR, page))
    rocket, flight = namespace["rocket"], namespace["flight"]
    printed = cli_block(page, "hpr sim validation/fixtures/ork/guides/level-1.ork")
    assert printed.splitlines()[1].startswith(f"{rocket.name} (level-1.ork)")
    apogee = labelled(printed, "apogee", "above the site")
    assert f" {flight.apogee_m:.1f} m (" in apogee
    assert apogee.endswith(f" at {flight.apogee_time_s:.2f} s")
    assert f" {flight.rail_exit_speed_m_s:.1f} m/s (" in labelled(printed, "rail exit speed")
    assert f" {flight.max_speed_m_s:.1f} m/s (" in labelled(printed, "top speed")
    assert labelled(printed, "top Mach number").endswith(f" {flight.max_mach:.3f}")
    landing = labelled(printed, "landing")
    assert f" {flight.landing['distance_m']:.1f} m (" in landing
    assert f" at {flight.landing['time_s']:.2f} s," in landing
    warned = re.findall(r"^warning: (\w+): issue #(\d+):", printed, re.MULTILINE)
    assert warned == [(w["kind"], str(w["issue"])) for w in flight.issue_warnings]
    assert len(warned) > 0


def test_the_package_readme_runs(repo):
    readme = (repo / "crates/hpr-py/README.md").read_text(encoding="utf-8")
    (code,) = [code for kind, code in BLOCK.findall(readme) if kind == "python"]
    out = io.StringIO()
    with contextlib.redirect_stdout(out):
        exec(compile(code, "crates/hpr-py/README.md", "exec"), {})
    apogee_m, highest_m = (float(value) for value in out.getvalue().split())
    assert 0.0 < highest_m <= apogee_m
