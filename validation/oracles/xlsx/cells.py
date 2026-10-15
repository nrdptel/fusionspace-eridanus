"""Every cell of the sample workbooks as openpyxl reads it, against `hpr_flightdata::workbook`'s text.

openpyxl (MIT) is run as an outside reader; none of its code is ported. `cargo xtask logs --sheets
DIR` writes each sample workbook of the `workbook-xlsx` family to `DIR/<id>.xlsx` and its sheets'
text, as `hpr_flightdata::workbook::sheets` writes it, to `DIR/<id>/<n>.csv`. This script opens
each workbook with openpyxl, values only (a formula's saved value), and compares, sheet by sheet
and row by row, every cell that holds a value:

- a number: the text, read as a double, must equal openpyxl's number exactly;
- a date or time openpyxl converts by the cell's number format: the text must read as the same
  date, time of day or duration, to the millisecond the text is rounded to (openpyxl rounds to
  the microsecond);
- a string: the same text, a line break in it read as a space, as the workbook reader writes it;
- a truth value: `TRUE` or `FALSE`.

An error value, an empty string and no value are gaps on both sides, and a row with no value is
left out on both. A cell openpyxl reads as a date where the text holds a number, or the other way
round, counts as a disagreement of its own. The workbooks are private (CONTRIBUTING.md rule 4), so
the script prints counts and cell positions, never a value; it writes the counts to OUT as JSON.

The environment is `requirements.txt` beside this script. Run from the repository root:

    uv venv -p 3.11 refs/venv-xlsx
    uv pip install -p refs/venv-xlsx -r validation/oracles/xlsx/requirements.txt
    cargo xtask logs --sheets refs/scratch/xlsx-sheets
    refs/venv-xlsx/bin/python -I validation/oracles/xlsx/cells.py refs/scratch/xlsx-sheets \\
        validation/oracles/xlsx/cells.json
"""

import csv
import datetime
import json
import pathlib
import re
import sys

import openpyxl

VERSION = "3.1.5"
# The text's own rounding is to the millisecond; openpyxl's to the microsecond.
TOLERANCE_S = 0.0005


def kept(cell):
    """The value of a cell that holds one, `None` for a gap (no value, an empty string, an error)."""
    if cell.data_type == "e" or cell.value is None or cell.value == "":
        return None
    return cell.value


def seconds(clock):
    """`H:MM:SS` or `H:MM:SS.fff` as seconds."""
    hours, minutes, rest = clock.split(":")
    return int(hours) * 3600 + int(minutes) * 60 + float(rest)


DATE = re.compile(r"^(\d{4})-(\d{2})-(\d{2})$")
CLOCK = re.compile(r"^\d+:\d{2}:\d{2}(\.\d+)?$")


def agrees(text, value):
    """Whether the workbook reader's text for a cell says what openpyxl's value does; its kind."""
    if isinstance(value, bool):
        return text == ("TRUE" if value else "FALSE"), "truth"
    if isinstance(value, (int, float)):
        try:
            return float(text) == float(value), "number"
        except ValueError:
            return False, "number-read-as-date"
    if isinstance(value, str):
        return text == value.replace("\r\n", " ").replace("\r", " ").replace("\n", " "), "string"
    if isinstance(value, datetime.timedelta):
        return CLOCK.match(text) is not None and abs(
            seconds(text) - value.total_seconds()
        ) <= TOLERANCE_S, "date"
    if isinstance(value, datetime.time):
        expected = value.hour * 3600 + value.minute * 60 + value.second + value.microsecond / 1e6
        return CLOCK.match(text) is not None and abs(seconds(text) - expected) <= TOLERANCE_S, "date"
    if isinstance(value, (datetime.datetime, datetime.date)):
        day, _, clock = text.partition(" ")
        match = DATE.match(day)
        if match is None or (clock and CLOCK.match(clock) is None):
            return False, "date-read-as-number"
        date = datetime.date(*(int(part) for part in match.groups()))
        stated = datetime.datetime.combine(date, datetime.time()) + datetime.timedelta(
            seconds=seconds(clock) if clock else 0
        )
        if not isinstance(value, datetime.datetime):
            value = datetime.datetime.combine(value, datetime.time())
        return abs((stated - value).total_seconds()) <= TOLERANCE_S, "date"
    return False, f"other:{type(value).__name__}"


def compare(book, folder):
    """One workbook's counts: sheets, cells compared by kind, and disagreements by kind."""
    workbook = openpyxl.load_workbook(book, data_only=True)
    counts = {"sheets": len(workbook.worksheets), "agree": {}, "disagree": {}, "where": []}
    texts = sorted(folder.glob("*.csv"), key=lambda path: int(path.stem))
    if len(texts) != len(workbook.worksheets):
        counts["disagree"]["sheet-count"] = 1
        return counts
    for n, (sheet, path) in enumerate(zip(workbook.worksheets, texts)):
        with open(path, newline="", encoding="utf-8") as handle:
            rows = [row for row in csv.reader(handle)]
        expected = []
        for row in sheet.iter_rows():
            cells = {cell.column: kept(cell) for cell in row if kept(cell) is not None}
            if cells:
                expected.append((row[0].row, cells))
        if len(rows) != len(expected):
            counts["disagree"]["row-count"] = counts["disagree"].get("row-count", 0) + 1
            counts["where"].append(f"sheet {n}: {len(rows)} rows against {len(expected)}")
            continue
        for written, (number, cells) in zip(rows, expected):
            columns = set(cells) | {i + 1 for i, text in enumerate(written) if text != ""}
            for column in sorted(columns):
                text = written[column - 1] if column <= len(written) else ""
                value = cells.get(column)
                if value is None or text == "":
                    ok, kind = False, "gap"
                else:
                    ok, kind = agrees(text, value)
                tally = counts["agree"] if ok else counts["disagree"]
                tally[kind] = tally.get(kind, 0) + 1
                if not ok and len(counts["where"]) < 20:
                    counts["where"].append(f"sheet {n}, row {number}, column {column}: {kind}")
    return counts


def main():
    folder, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    if openpyxl.__version__ != VERSION:
        sys.exit(f"openpyxl {openpyxl.__version__}, not the pinned {VERSION}")
    books = {}
    for book in sorted(folder.glob("*.xlsx")):
        books[book.stem] = compare(book, folder / book.stem)
    total = {"agree": 0, "disagree": 0}
    for counts in books.values():
        for key in total:
            total[key] += sum(counts[key].values())
    report = {"openpyxl": VERSION, "workbooks": len(books), "cells": total, "by_workbook": books}
    out.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    for book, counts in books.items():
        print(book, counts["sheets"], "sheets; agree", counts["agree"], "disagree", counts["disagree"])
        for where in counts["where"]:
            print("   ", where)
    print(f"{len(books)} workbooks: {total['agree']} cells agree, {total['disagree']} disagree")
    sys.exit(1 if total["disagree"] else 0)


if __name__ == "__main__":
    main()
