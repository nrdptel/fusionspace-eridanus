# ADR-116: M4.3c: drag and wind as Python functions (2026-09-29)

- **Status:** accepted
- **Summary:** M4.3c: Python drag `f(mach, thrusting)` and wind `f(height_m)`; a function's exception kept and raised in place of the library's error

**Context.** M4.3c's *done when*: a drag and a wind written as Python functions fly, their
exceptions reach Python, and a flight with Python's constant drag equals a table's. The Rust
library already takes both as traits (`hpr_aero::DragModel`, `hpr_atmos::Wind`); a flight runs
without the GIL (ADR-114), and a trait's error is the library's own type, which can't carry a
Python exception.

**Decision.**

1. `Flight(..., drag=f)` calls `f(mach, thrusting)` and takes a float, `C_D0` on the rocket's
   reference area. RocketPy's drag functions take the Mach number only; `thrusting` is the one
   other input a drag table reads, so a function can say what a power-on and a power-off table
   say. The Reynolds number and hpr's own buildup (`DragQuery`) stay Rust's: a query object
   would hold references into the flight, and nothing in M4.3 needs them.
2. `Environment(..., wind=f)` calls `f(height_m)` with the height above sea level, as the Rust
   `Wind` is asked, and takes `(east_m_s, north_m_s)`. Mean wind is horizontal (`wind.rs`), so
   there is no up component. A value that isn't finite is refused.
3. Each call takes the GIL with `Python::attach` and gives it back. An exception is kept in one
   slot per flight, shared by its drag and its wind (the environment keeps the function and binds
   it to the flight's slot as the flight starts), and the library gets an error of its own kind.
   The integrator retries a failed evaluation with a shorter step (`integrator.rs`), which would
   drop an exception at a trial state, and Ctrl-C with it; so the slot is sticky: once it holds
   one, both functions answer with an error and no call, and `Flight` raises the kept exception
   whatever the library's flight returned, so `except LookupError` works as the caller wrote it.
4. A function beside its constant counterpart (`drag` with `drag_table`, `wind` with
   `wind_speed_m_s`) is refused rather than one silently winning.

**Consequences.** `test_models.py` flies both and holds a constant function's flight of 0.5 to a
table's bit for bit, every recorded column. At 0.3 or 0.45 the apogees differ by up to 4e-11 of
themselves: a table's linear interpolation `(1 - t) y0 + t y1` (`interp.rs`) rounds a constant
row, so the table flies a neighbouring float; the tests hold those to 1e-10. Exceptions at any
call, `KeyboardInterrupt`, and eight threads sharing one windy environment are tested. An
atmosphere in Python is not in M4.3.
