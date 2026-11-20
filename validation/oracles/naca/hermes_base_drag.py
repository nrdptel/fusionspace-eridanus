"""Digitize the subsonic base drag coefficient curves of NACA RM SL55F15, figure 20.

Source: H. Herbert Jackson, "Longitudinal Aerodynamic Characteristics and Effect of Rocket Jet on
Drag of Models of the Hermes A-3A and A-3B Missiles in Free Flight at Mach Numbers from 0.6 to
2.0", NACA RM SL55F15, 1955 (title from PDF page 2). A NACA report: US-government work, public
domain. Figure 20 (PDF page 41) plots total and base drag coefficient against Mach number for the
A drag models; this script reads the two base drag curves drawn below Mach 0.8:

- "A(1), power on" (solid line), and
- "Power off" (one long dash and two short dashes).

Both are faired curves, not data points (report p. 10 and p. 14, PDF pages 11 and 15).

Method (reproducible, no hand-picked pixels besides one seed per curve in plot coordinates):

1. Render the page with `pdftoppm -r 300 -gray` (the page holds one 300 dpi bilevel scan, so
   300 dpi is the native resolution) and threshold at grey level 128.
2. Mach axis. The graph paper has heavy vertical lines every 0.025 Mach; the plot frame runs
   from the labelled Mach 0.6 line to the labelled 1.8 line (48 intervals). Heavy-line centres
   are found from column darkness inside the row band where the curves lie and placed on a
   lattice of 48 intervals between the frame edges, and M(x) is the least-squares line through
   the lines at Mach 0.6, 0.7 and 0.8 (the labelled ones in range). Residuals of the other lines
   are reported.
3. C_D axis. Heavy horizontal lines every 0.025; the frame runs from the labelled -0.1 line to
   the labelled 0.4 line (20 intervals). The scan is turned about half a degree, so the axis is
   calibrated locally: for each column read, the heavy-line centres are found in a 121 px wide
   strip around it, and C_D(y) is the line through the labelled C_D = 0 and 0.1 lines there.
   The minor (dotted) lines are counted between two heavy lines and reported.
4. Curves. Columns within 3 px of a heavy vertical line are skipped (every target Mach but 0.79
   lies on one). In each other column the dark runs are listed. A run clear of the heavy
   horizontal lines counts if it is horizontally continuous (dark in 7 of the 9 columns around
   it), which rejects the crosses of the dotted minor grid. A run merged with a heavy line gives
   the curve's centre from its edge that sticks out past the line, offset by half the curve's
   measured width ('edge' columns, counted apart). From a seed point each curve is tracked
   column by column, taking the candidate nearest a line fitted to the last points (within
   3 px); a curve missing for 25 px ends. At each target Mach the curve's row is the
   least-squares line through the tracked centres within +-0.0125 Mach; for the dashed
   power-off curve this interpolates across its gaps. A target at most 6 px (0.003 Mach) past a
   curve's first column is read one-sided and flagged. A curve's start is its first column.

Usage: python3 hermes_base_drag.py PATH/TO/naca-rm-sl55f15.pdf [--page 41] > out.json
Needs pdftoppm (poppler) on PATH, numpy and Pillow.
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np
from PIL import Image

DPI = 300
TARGET_MACH = [0.60, 0.65, 0.70, 0.75, 0.79]
# Seeds in plot coordinates (Mach, C_D), from a visual read of the figure; tracking starts here.
SEEDS = {
    "a1_power_on": (0.62, 0.028),
    "power_off": (0.68, 0.0115),
}
HALF_WINDOW_MACH = 0.0125


def render(pdf: Path, page: int) -> np.ndarray:
    exe = shutil.which("pdftoppm")
    if exe is None:
        sys.exit("pdftoppm not found on PATH")
    with tempfile.TemporaryDirectory() as tmp:
        stem = Path(tmp) / "p"
        subprocess.run(
            [exe, "-r", str(DPI), "-gray", "-f", str(page), "-l", str(page), str(pdf), str(stem)],
            check=True,
        )
        (out,) = list(Path(tmp).glob("p*.pgm"))
        return np.array(Image.open(out)) < 128


def centres(profile: np.ndarray, threshold: float, offset: int) -> list[float]:
    """Darkness-weighted centres of runs of entries above threshold."""
    out, i, n = [], 0, len(profile)
    while i < n:
        if profile[i] > threshold:
            j = i
            while j < n and profile[j] > threshold:
                j += 1
            w = profile[i:j].astype(float)
            out.append(offset + i + float((np.arange(i, j) * w).sum() / w.sum()) - i)
            i = j
        else:
            i += 1
    return out


def lattice(found: list[float], intervals: int, label: str) -> np.ndarray:
    """Assign found line centres to an evenly spaced lattice between the first and last line."""
    lo, hi = found[0], found[-1]
    pitch = (hi - lo) / intervals
    pos = np.full(intervals + 1, np.nan)
    for c in found:
        k = round((c - lo) / pitch)
        if abs(c - (lo + k * pitch)) < 0.25 * pitch and 0 <= k <= intervals:
            if np.isnan(pos[k]) or abs(c - (lo + k * pitch)) < abs(pos[k] - (lo + k * pitch)):
                pos[k] = c
    missing = int(np.isnan(pos).sum())
    if missing > intervals // 4:
        sys.exit(f"{label}: {missing} of {intervals + 1} heavy lines not found")
    return pos


def mach_axis(img: np.ndarray, band: tuple[int, int]) -> dict:
    sub = img[band[0] : band[1], :]
    prof = sub.sum(0)
    found = centres(prof, 0.6 * sub.shape[0], 0)
    found = [c for c in found if 200 < c < img.shape[1] - 200]
    pos = lattice(found, 48, "Mach axis")
    ks = np.array([0, 4, 8])  # Mach 0.6, 0.7, 0.8
    xs = pos[ks]
    ms = 0.6 + 0.025 * ks
    a, b = np.polyfit(xs, ms, 1)  # M = a x + b
    resid = {}
    for k in (12, 16, 20, 24):  # Mach 0.9, 1.0, 1.1, 1.2: check lines
        if not np.isnan(pos[k]):
            resid[f"{0.6 + 0.025 * k:.1f}"] = round(float(a * pos[k] + b - (0.6 + 0.025 * k)), 5)
    return {
        "a": a,
        "b": b,
        "pitch_px": float((found[-1] - found[0]) / 48),
        "x_mach_0.6_0.7_0.8": [round(float(x), 1) for x in xs],
        "frame_x": [round(found[0], 1), round(found[-1], 1)],
        "check_residual_mach": resid,
        "lines": [float(v) for v in pos if not np.isnan(v)],
    }


def cd_axis_at(img: np.ndarray, x: float, y_range: tuple[int, int], x_min: int) -> dict:
    x0 = max(int(x) - 60, x_min)
    x1 = x0 + 121
    sub = img[y_range[0] : y_range[1], x0:x1]
    prof = sub.sum(1)
    found = centres(prof, 0.7 * sub.shape[1], y_range[0])
    # Top frame is C_D = 0.4 (k = 0), bottom frame is C_D = -0.1 (k = 20).
    pos = lattice(found, 20, f"C_D axis at x={x:.0f}")
    y0, y1 = pos[16], pos[12]  # C_D = 0 and 0.1
    if np.isnan(y0) or np.isnan(y1):
        sys.exit(f"C_D 0 or 0.1 line not found at x={x:.0f}")
    c = 0.1 / (y0 - y1)  # C_D per px, upward positive
    heavy = pos[~np.isnan(pos)]
    return {"y0": float(y0), "y01": float(y1), "per_px": c, "heavy": heavy}


def minor_count(img: np.ndarray, x: float, y_top: float, y_bot: float, axis: str) -> int:
    """Count dotted minor lines strictly between two heavy lines."""
    if axis == "h":
        sub = img[int(y_top) + 4 : int(y_bot) - 3, int(x) - 60 : int(x) + 61]
        prof = sub.sum(1)
        thr = 0.12 * sub.shape[1]
    else:
        sub = img[int(y_top) : int(y_bot), int(x) + 4 : int(x) + 47]
        prof = sub.sum(0)
        thr = 0.12 * sub.shape[0]
    return len(centres(prof, thr, 0))


def column_candidates(img, x, heavy, y_lo, y_hi, half_w):
    """Candidate curve centres in one column, as (row, kind) with kind 'clear' or 'edge'.

    A run of dark pixels that touches no heavy horizontal line is a clear candidate if it is
    horizontally continuous (dark in at least 7 of the 9 columns x-4..x+4 at its centre row),
    which rejects the small crosses of the dotted minor grid. A run that merges with a heavy
    line (taken as its centre +-1.5 px) gives the curve's centre from the run's edge that sticks
    out past the line, offset by the curve's half-width half_w ('edge' candidates).
    """
    col = img[y_lo:y_hi, x]
    out, i, n = [], 0, len(col)
    while i < n:
        if not col[i]:
            i += 1
            continue
        j = i
        while j < n and col[j]:
            j += 1
        t, b = y_lo + i, y_lo + j - 1
        i = j
        if b - t + 1 < 2:
            continue
        grids = [g for g in heavy if t - 1 <= g <= b + 1]
        if not grids:
            c = (t + b) / 2
            r = int(round(c))
            win = img[r - 1 : r + 2, x - 4 : x + 5].any(axis=0)
            if win.sum() >= 7:
                out.append((c, "clear"))
            continue
        g = grids[0]
        g0, g1 = g - 1.5, g + 1.5
        above, below = t < g0 - 0.5, b > g1 + 0.5
        if above and below:
            out.append(((t + b) / 2, "edge"))
        elif above:
            out.append((t + half_w, "edge"))
        elif below:
            out.append((b - half_w, "edge"))
    return out


def track(img, seed_x, seed_y, x_stop_left, x_stop_right, axis_at, vlines, y_lo, y_hi, half_w):
    """Track a curve both ways from a seed; returns {x: (row, kind)} for columns read.

    The seed is first snapped to the nearest candidate within 8 px, in the first column at or
    right of the seed that has one, so the result does not hang on the seed's exact row.
    """
    for _ in range(40):
        if min(abs(seed_x - v) for v in vlines) > 3:
            cands = column_candidates(img, seed_x, axis_at(seed_x)["heavy"], y_lo, y_hi, half_w)
            near = [c for c in cands if abs(c[0] - seed_y) <= 8]
            if near:
                seed_y = min(near, key=lambda c: abs(c[0] - seed_y))[0]
                break
        seed_x += 1
    else:
        sys.exit(f"no curve near the seed at x={seed_x}")
    pts = {}
    for direction, stop in ((-1, x_stop_left), (1, x_stop_right)):
        hist = [(seed_x, seed_y)]
        miss = 0
        x = seed_x
        while (x - stop) * direction < 0:
            x += direction
            if min(abs(x - v) for v in vlines) <= 3:
                continue  # column lies on a heavy vertical grid line
            if len(hist) >= 6:
                hx, hy = zip(*hist[-40:])
                pred = np.polyval(np.polyfit(hx, hy, 1), x)
            else:
                pred = hist[-1][1]
            cands = column_candidates(img, x, axis_at(x)["heavy"], y_lo, y_hi, half_w)
            near = [c for c in cands if abs(c[0] - pred) <= 3]
            if near:
                y, kind = min(near, key=lambda c: abs(c[0] - pred))
                pts[x] = (y, kind)
                hist.append((x, y))
                miss = 0
            else:
                miss += 1
                if miss > 25:  # 25 px (0.0125 Mach) without the curve: it ends here
                    break
    return pts


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("pdf", type=Path)
    ap.add_argument("--page", type=int, default=41)
    args = ap.parse_args()
    img = render(args.pdf, args.page)

    # Coarse frame: rows of the plot from the C_D axis, columns from the Mach axis.
    band = (1330, 1500)  # rows near C_D 0 to 0.06, where the subsonic curves lie
    m = mach_axis(img, band)
    x_of = lambda mach: (mach - m["b"]) / m["a"]  # noqa: E731
    y_range = (600, 1720)
    cache: dict[int, dict] = {}

    def axis_at(x: float) -> dict:
        key = int(round(x / 10.0)) * 10
        if key not in cache:
            cache[key] = cd_axis_at(img, key, y_range, int(m["frame_x"][0]) + 4)
        return cache[key]

    def cd(x: float, y: float) -> float:
        ax = axis_at(x)
        return (ax["y0"] - y) * ax["per_px"]

    def y_of(x: float, c: float) -> float:
        ax = axis_at(x)
        return ax["y0"] - c / ax["per_px"]

    ax07 = axis_at(x_of(0.7))
    heavy = ax07["heavy"]
    minors_h = minor_count(img, x_of(0.71) - 30, heavy[15], heavy[16], "h")
    minors_v = minor_count(img, x_of(0.7), band[0], band[1], "v")

    out = {
        "source": "NACA RM SL55F15 (Jackson 1955), figure 20, PDF page %d" % args.page,
        "render": {"tool": "pdftoppm", "dpi": DPI, "threshold": 128},
        "mach_axis": {
            "heavy_line_pitch_px": round(m["pitch_px"], 2),
            "heavy_line_step_mach": 0.025,
            "x_px_at_mach_0.6_0.7_0.8": m["x_mach_0.6_0.7_0.8"],
            "frame_x_px_mach_0.6_and_1.8": m["frame_x"],
            "check_line_residual_mach": m["check_residual_mach"],
            "minor_dotted_lines_between_heavy": minors_v,
        },
        "cd_axis_at_mach_0.7": {
            "y_px_cd_0": round(ax07["y0"], 1),
            "y_px_cd_0.1": round(ax07["y01"], 1),
            "px_per_0.1": round(0.1 / ax07["per_px"], 1),
            "heavy_line_step_cd": 0.025,
            "minor_dotted_lines_between_heavy": minors_h,
        },
        "curves": {},
    }
    y_lo, y_hi = int(y_of(x_of(0.7), 0.06)), int(y_of(x_of(0.7), -0.01))
    win_px = HALF_WINDOW_MACH / m["a"]

    def thickness(x: int, y: float) -> int:
        r = int(round(y))
        if not img[r, x]:
            return 0
        t = r
        while img[t - 1, x]:
            t -= 1
        b = r
        while img[b + 1, x]:
            b += 1
        return b - t + 1

    def fit_at(xs, ys, xt):
        sel = np.abs(xs - xt) <= win_px
        # A target up to 6 px (0.003 Mach) past the curve's first read column (the plot frame,
        # which is masked) is read by a one-sided fit; a curve starting later is not read.
        if sel.sum() < 10 or xs.min() > xt + 6:
            return None, None, sel
        p = np.polyfit(xs[sel], ys[sel], 1)
        return float(np.polyval(p, xt)), ys[sel] - np.polyval(p, xs[sel]), sel

    for name, (sm, sc) in SEEDS.items():
        sx = int(round(x_of(sm)))
        x_left, x_right = int(x_of(0.6)) + 4, int(x_of(0.82))
        args_t = (sx, y_of(sx, sc), x_left, x_right, axis_at, m["lines"], y_lo, y_hi)
        pts = track(img, *args_t, 2.0)
        widths = [thickness(x, y) for x, (y, k) in pts.items() if k == "clear"]
        width = float(np.median(widths))
        pts = track(img, *args_t, (width - 1) / 2)
        xs = np.array(sorted(pts))
        ys = np.array([pts[x][0] for x in xs])
        clear = np.array([pts[x][1] == "clear" for x in xs])
        x0 = int(xs.min())
        head = xs <= x0 + win_px  # the first 0.0125 Mach of the curve
        start_row = float(np.polyval(np.polyfit(xs[head], ys[head], 1), x0))
        readings, resid_all = {}, []
        for tm in TARGET_MACH:
            xt = x_of(tm)
            y_all, r, sel = fit_at(xs, ys, xt)
            if y_all is None:
                readings[f"{tm:.2f}"] = None
                continue
            resid_all.extend(r.tolist())
            y_clr, _, sel_c = fit_at(xs[clear], ys[clear], xt)
            readings[f"{tm:.2f}"] = {
                "cd_b": round(cd(xt, y_all), 4),
                "cd_b_clear_columns_only": None if y_clr is None else round(cd(xt, y_clr), 4),
                "columns_clear": int((sel & clear).sum()),
                "columns_edge": int((sel & ~clear).sum()),
                "one_sided": bool(xs[sel].min() > xt - 2 or xs[sel].max() < xt + 2),
            }
        out["curves"][name] = {
            "label": "A(1), power on (solid)" if name == "a1_power_on" else "Power off (one long dash and two short dashes)",
            "line_width_px_median": width,
            "first_column_mach": round(float(m["a"] * xs.min() + m["b"]), 4),
            "first_column_cd_b": round(cd(xs.min(), ys[0]), 4),
            "cd_b_at_first_column_fitted": round(cd(x0, start_row), 4),
            "left_end_is_plot_frame": bool(xs.min() <= x_of(0.6) + 6),
            # Columns within 3 px of a heavy vertical line are skipped, so a first column 4 px
            # right of one means the curve may begin under that line: name the line's Mach.
            "first_column_next_to_grid_line_mach": next(
                (round(float(m["a"] * v + m["b"]), 3) for v in m["lines"] if 0 < xs.min() - v <= 4),
                None,
            ),
            "last_column_mach": round(float(m["a"] * xs.max() + m["b"]), 4),
            "columns_tracked": int(len(xs)),
            "fit_residual_rms_px": round(float(np.sqrt(np.mean(np.square(resid_all)))), 2),
            "readings": readings,
        }
    out["px_to_cd"] = round(ax07["per_px"], 6)
    json.dump(out, sys.stdout, indent=2)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
