"""Every cell of the sample `.xls` exports as xlrd reads it, against `hpr_flightdata::xls`'s text.

xlrd (BSD-3-Clause) is run as an outside reader; none of its code is ported. `cargo xtask logs
--sheets DIR` writes each sample `.xls` export of the `altimax-xls` family to `DIR/<id>.xls` and
its sheet's text, as `hpr_flightdata::xls::sheets` writes it, to `DIR/<id>/0.csv`. This script
opens each file with xlrd and compares, row by row, every cell that holds a value:

- a number: the text, read as a double, must equal xlrd's number exactly;
- a string: the same text;
- a truth value: `TRUE` or `FALSE`.

An error value, an empty string and no value are gaps on both sides, and a row with no value is
left out on both. xlrd holds a sheet of Excel's versions before BIFF8 to 16,384 rows
(`Sheet.utter_max_rows`), those versions' own limit, and the one sample has 25,335; the script
raises the bound to 65,536, BIFF8's, which changes no cell xlrd decodes. The files are private
(CONTRIBUTING.md rule 4), so the script prints counts, cell positions and the version xlrd reads
each file as, never a value; it writes them to OUT as JSON.

The environment is `requirements.txt` beside this script. Run from the repository root:

    uv venv -p 3.11 refs/venv-xls
    uv pip install -p refs/venv-xls -r validation/oracles/xls/requirements.txt
    cargo xtask logs --sheets refs/scratch/xls-sheets
    refs/venv-xls/bin/python -I validation/oracles/xls/cells.py refs/scratch/xls-sheets \\
        validation/oracles/xls/cells.json
"""

import csv
import json
import os
import pathlib
import sys

import xlrd
import xlrd.sheet

VERSION = "2.0.2"
ROWS = 65536

_init = xlrd.sheet.Sheet.__init__


def _raised(self, *args, **kwargs):
    """xlrd's sheet, its bound on rows raised to BIFF8's."""
    _init(self, *args, **kwargs)
    self.utter_max_rows = ROWS


xlrd.sheet.Sheet.__init__ = _raised


def kept(cell):
    """The value of a cell that holds one, `None` for a gap (no value, an empty string, an error)."""
    if cell.ctype in (xlrd.XL_CELL_EMPTY, xlrd.XL_CELL_BLANK, xlrd.XL_CELL_ERROR):
        return None
    if cell.ctype == xlrd.XL_CELL_TEXT and cell.value == "":
        return None
    return cell


def agrees(text, cell):
    """Whether the reader's text for a cell says what xlrd's value does; its kind."""
    if cell.ctype == xlrd.XL_CELL_BOOLEAN:
        return text == ("TRUE" if cell.value else "FALSE"), "truth"
    if cell.ctype in (xlrd.XL_CELL_NUMBER, xlrd.XL_CELL_DATE):
        try:
            return float(text) == float(cell.value), "number"
        except ValueError:
            return False, "number"
    if cell.ctype == xlrd.XL_CELL_TEXT:
        return text == cell.value, "string"
    return False, f"other:{cell.ctype}"


def compare(book, folder):
    """One file's counts: sheets, the version xlrd reads, cells compared by kind, disagreements."""
    with open(os.devnull, "w") as quiet:
        workbook = xlrd.open_workbook(book, logfile=quiet)
    counts = {
        "sheets": workbook.nsheets,
        "biff_version": workbook.biff_version,
        "agree": {},
        "disagree": {},
        "where": [],
    }
    texts = sorted(folder.glob("*.csv"), key=lambda path: int(path.stem))
    if len(texts) != workbook.nsheets:
        counts["disagree"]["sheet-count"] = 1
        return counts
    for n, (sheet, path) in enumerate(zip(workbook.sheets(), texts)):
        with open(path, newline="", encoding="utf-8") as handle:
            rows = list(csv.reader(handle))
        expected = []
        for number in range(sheet.nrows):
            cells = {
                column + 1: cell
                for column, cell in enumerate(sheet.row(number))
                if kept(cell) is not None
            }
            if cells:
                expected.append((number + 1, cells))
        if len(rows) != len(expected):
            counts["disagree"]["row-count"] = 1
            counts["where"].append(f"sheet {n}: {len(rows)} rows against {len(expected)}")
            continue
        for written, (number, cells) in zip(rows, expected):
            columns = set(cells) | {i + 1 for i, text in enumerate(written) if text != ""}
            for column in sorted(columns):
                text = written[column - 1] if column <= len(written) else ""
                cell = cells.get(column)
                if cell is None or text == "":
                    ok, kind = False, "gap"
                else:
                    ok, kind = agrees(text, cell)
                tally = counts["agree"] if ok else counts["disagree"]
                tally[kind] = tally.get(kind, 0) + 1
                if not ok and len(counts["where"]) < 20:
                    counts["where"].append(f"sheet {n}, row {number}, column {column}: {kind}")
    return counts


def main():
    folder, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    if xlrd.__version__ != VERSION:
        sys.exit(f"xlrd {xlrd.__version__}, not the pinned {VERSION}")
    books = {}
    for book in sorted(folder.glob("*.xls")):
        books[book.stem] = compare(book, folder / book.stem)
    total = {"agree": 0, "disagree": 0}
    for counts in books.values():
        for key in total:
            total[key] += sum(counts[key].values())
    report = {"xlrd": VERSION, "files": len(books), "cells": total, "by_file": books}
    out.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    for book, counts in books.items():
        print(book, counts["sheets"], "sheets, BIFF", counts["biff_version"], "; agree",
              counts["agree"], "disagree", counts["disagree"])
        for where in counts["where"]:
            print("   ", where)
    print(f"{len(books)} files: {total['agree']} cells agree, {total['disagree']} disagree")
    sys.exit(1 if total["disagree"] else 0)


if __name__ == "__main__":
    main()
