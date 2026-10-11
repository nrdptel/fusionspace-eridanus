"""The steps to a first answer, run (M0.11, ADR-241): the scoreboard counts HPR Sim's commands and
script lines in `validation/steps/`, and these tests run them, so each counted step works and
gives the answer its task names: the design read, its apogee, and where it may land.
"""

import contextlib
import io
import re
import shlex
import subprocess

STEPS = "validation/steps"
TASK = re.compile(r"# task: (.+)")


def tasks_in(text):
    """The tasks a step file's `# task:` lines name, in order."""
    tasks = []
    for line in text.splitlines():
        if match := TASK.fullmatch(line):
            tasks += [task.strip() for task in match.group(1).split(",")]
    return tasks


def test_the_command_line_steps_work(repo, hpr_cli):
    text = (repo / STEPS / "hpr-command-line.txt").read_text(encoding="utf-8")
    assert tasks_in(text) == ["open", "fly", "spread"]
    commands = [line for line in text.splitlines() if line and not line.startswith("#")]
    assert len(commands) == 2
    outputs = []
    for command in commands:
        words = shlex.split(command)
        assert words[0] == "hpr"
        done = subprocess.run([str(hpr_cli), *words[1:]], cwd=repo, capture_output=True,
                              text=True, encoding="utf-8", check=False)
        assert done.returncode == 0, done.stderr
        outputs.append(done.stdout)
    sim, mc = outputs
    assert "Level 1 guide rocket (level-1.ork)" in sim
    assert re.search(r"^static margin +\d", sim, re.MULTILINE)
    assert re.search(r"^apogee +\d+\.\d m", sim, re.MULTILINE)
    assert "100 of 100 flights landed" in mc
    assert re.search(r"^95% +\d+\.\d m", mc, re.MULTILINE), "the 95% landing ellipse"


def test_the_python_steps_work(repo, monkeypatch):
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
    match = re.fullmatch(r"landing spread (\d+\.\d) m, one standard deviation", spread)
    assert match and float(match.group(1)) > 0.0
    assert namespace["run"].flown == namespace["run"].runs == 100
