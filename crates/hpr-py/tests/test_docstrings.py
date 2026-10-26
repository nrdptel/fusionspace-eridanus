"""The package's docstrings as `help()` shows them, pinned in `docstrings.md` beside this file
(ADR-292's amendment for M0.14d4), so `cargo xtask voice` can hold them as it holds a command's
help.

The snapshot is Markdown: the module's docstring under its name, then each name in `__all__` in
order, and under each class each public attribute of its own (methods, properties, class and
static methods) in name order, each under a heading of its dotted name in backticks. A docstring is
read as `inspect.cleandoc` leaves it, the text `help()` shows below the signature. Every public
name needs one, so a new method can't reach `help()` with nothing to say.

With `HPR_BLESS=1` in the environment the test writes the snapshot instead of failing on it
(`HPR_BLESS=1 scripts/python-tests.sh 3.13`); read the diff before committing it.
"""

import inspect
import os
from pathlib import Path

from fusionspace import hpr

SNAPSHOT = Path(__file__).with_name("docstrings.md")
BLESS = os.environ.get("HPR_BLESS") == "1"
# Names in `__all__` that aren't documented objects: the version is a string.
NOT_DOCUMENTED = {"__version__"}


def documented(obj, name):
    """`obj`'s docstring as `help()` shows it; a public name without one fails."""
    doc = obj.__doc__
    assert isinstance(doc, str) and doc.strip(), f"{name} has no docstring"
    return inspect.cleandoc(doc)


def own_attributes(cls):
    """A class's own public attributes, in name order: what `help()` lists under the class, less
    what it inherits and the special methods Python documents itself."""
    # Read through the class, as `help()` does: a static or class method's entry in `vars` is its
    # wrapper, whose docstring is Python's own.
    return [(name, getattr(cls, name)) for name in sorted(vars(cls)) if not name.startswith("_")]


def snapshot():
    """The snapshot's text, from the installed package."""
    parts = [f"# `fusionspace.hpr`\n\n{documented(hpr, 'fusionspace.hpr')}\n"]
    for name in hpr.__all__:
        if name in NOT_DOCUMENTED:
            continue
        obj = getattr(hpr, name)
        parts.append(f"## `{name}`\n\n{documented(obj, name)}\n")
        if inspect.isclass(obj):
            for attribute, value in own_attributes(obj):
                dotted = f"{name}.{attribute}"
                parts.append(f"### `{dotted}`\n\n{documented(value, dotted)}\n")
    return "\n".join(parts)


def test_the_docstrings_match_their_snapshot():
    text = snapshot()
    if BLESS:
        SNAPSHOT.write_text(text, encoding="utf-8", newline="\n")
        return
    assert SNAPSHOT.is_file(), f"{SNAPSHOT} is missing: write it with HPR_BLESS=1"
    assert SNAPSHOT.read_text(encoding="utf-8") == text, (
        f"the docstrings differ from {SNAPSHOT.name}: write it again with HPR_BLESS=1 and read the diff"
    )
