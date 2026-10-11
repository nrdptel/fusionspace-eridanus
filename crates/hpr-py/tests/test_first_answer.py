"""The steps to a first answer, run (M0.11, ADR-241): the scoreboard counts HPR Sim's commands and
script lines in `validation/steps/`, and these tests run them, so each counted step works and
gives the answer its task names: the design read, its apogee, and where it may land.
"""

import ast
import contextlib
import io
import math
import re
import shlex
import subprocess

STEPS = "validation/steps"
TASK = re.compile(r"# task: (.+)")
# The landing ellipse's design: `hpr mc`'s options and the script's, the same flights.
DESIGN = "validation/fixtures/ork/guides/level-1.ork"
MC = ["mc", DESIGN, "--wind", "4", "--wind-from", "270", "--wind-sd", "0.25", "--wind-from-sd", "15"]


def tasks_in(text):
    """The tasks a step file's `# task:` lines name, in order."""
    tasks = []
    for line in text.splitlines():
        if match := TASK.fullmatch(line):
            tasks += [task.strip() for task in match.group(1).split(",")]
    return tasks


def counted(text):
    """The lines the scoreboard counts: each not blank and not a comment."""
    return [line for line in text.splitlines() if line.strip() and not line.startswith("#")]


def run_hpr(hpr_cli, repo, words):
    done = subprocess.run([str(hpr_cli), *words], cwd=repo, capture_output=True, text=True,
                          encoding="utf-8", check=False)
    assert done.returncode == 0, done.stderr
    return done.stdout


def ellipse_95(mc_text):
    """`hpr mc`'s 95% landing ellipse: its semi-major and semi-minor axes, m."""
    match = re.search(r"^95% +(\d+\.\d) m \(\d+ ft\) +(\d+\.\d) m", mc_text, re.MULTILINE)
    assert match, mc_text
    return float(match.group(1)), float(match.group(2))


def test_the_command_line_steps_work(repo, hpr_cli):
    text = (repo / STEPS / "hpr-command-line.txt").read_text(encoding="utf-8")
    assert tasks_in(text) == ["open", "fly", "spread"]
    commands = counted(text)
    assert len(commands) == 2
    outputs = []
    for command in commands:
        words = shlex.split(command)
        assert words[0] == "hpr"
        outputs.append(run_hpr(hpr_cli, repo, words[1:]))
    sim, mc = outputs
    assert "Level 1 guide rocket (level-1.ork)" in sim
    assert re.search(r"^static margin +\d", sim, re.MULTILINE)
    assert re.search(r"^apogee +\d+\.\d m", sim, re.MULTILINE)
    assert "100 of 100 flights landed" in mc
    assert shlex.split(commands[1])[1:] == MC, "the ellipse the Python test compares with"
    ellipse_95(mc)


def test_each_script_line_is_one_simple_statement(repo):
    """The count is of statements: Python's own parser finds one statement on each counted line,
    none of them a block, so lines can't be joined with `;` or a one-line `if` to count fewer."""
    code = (repo / STEPS / "hpr-python.py").read_text(encoding="utf-8")
    lines = counted(code)
    body = ast.parse(code).body
    assert len(body) == len(lines)
    compound = (ast.If, ast.For, ast.AsyncFor, ast.While, ast.With, ast.AsyncWith, ast.Try,
                ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef, ast.Match)
    for statement, line in zip(body, lines):
        assert statement.lineno == statement.end_lineno, line
        assert not isinstance(statement, compound), line
        assert code.splitlines()[statement.lineno - 1] == line


def test_the_python_steps_work(repo, hpr_cli, monkeypatch):
    path = repo / STEPS / "hpr-python.py"
    code = path.read_text(encoding="utf-8")
    assert tasks_in(code) == ["open", "fly", "spread"]
    monkeypatch.chdir(repo)
    out = io.StringIO()
    namespace = {}
    with contextlib.redirect_stdout(out):
        exec(compile(code, str(path), "exec"), namespace)
    assert namespace["rocket"].configuration
    apogee, spread = out.getvalue().splitlines()
    assert re.fullmatch(r"apogee \d+\.\d m", apogee)
    printed = re.fullmatch(r"landing ellipse, one standard deviation: (\d+\.\d) m by (\d+\.\d) m",
                           spread)
    assert printed
    run = namespace["run"]
    assert run.flown == run.runs == 100
    # The same flights as `hpr mc`'s, so the same ellipse: its 95% axes are the standard
    # deviations scaled by sqrt(-2 ln 0.05), each within the 0.05 m the command line rounds to.
    minor, major = namespace["axes"]
    assert printed.groups() == (f"{major:.1f}", f"{minor:.1f}"), "the major axis first"
    scale = math.sqrt(-2.0 * math.log(0.05))
    semi_major, semi_minor = ellipse_95(run_hpr(hpr_cli, repo, MC))
    assert abs(major * scale - semi_major) <= 0.05 + 1e-9
    assert abs(minor * scale - semi_minor) <= 0.05 + 1e-9
