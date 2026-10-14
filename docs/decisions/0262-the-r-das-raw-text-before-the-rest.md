# ADR-262: M7.1c8 in two: the R-DAS's raw text first; the rest to M7.1c9 (2026-10-13)

- **Status:** accepted; narrows M7.1c8 to a first increment and moves the rest, with M7.1c8's old entry and *done when* (the parent's), to M7.1c9; follows [ADR-261](0261-archives-and-two-readers-variants-before-the-rest.md)
- **Summary:** After M7.1c7, 209 sample logs were left to M7.1c8, in families that each need a decoder or the column mapper. The largest, 54 files, is one layout: the raw text AED Electronics' R-DAS flight computer's Windows software exports, a configuration summary and then a table of the converters' counts. M7.1c8 now reads it. The file holds no calibration, so every count is kept as written and the record holds no altitude; the clock is held to the configuration's stated rate. The rest goes to M7.1c9.

**Context.** `cargo xtask logs` on main after M7.1c7 (#530) leaves 209 files, in families such
as AltOS's `.eeprom` (14) and telemetry (2), a radio telemetry stream saved as RTF (2), an
ARTS2's raw counts and its `.odf` files (6), Missile Works' RTx files (5), spreadsheets (24),
binary files (4) and generic CSV and text (93, 2 of them read already). One family, filed by the collection's catalog
under the name of the course whose flights it holds, is 54 files of a single layout, with no
variation among them:

```text
*** Configuration ***
Trigger: G-switch.
Data is sampled at 200 Hz.
Inputs that were sampled: Acceleration, Pressure, the digital inputs, ADC0, … ADC5.
This data was generated with version 4.1 of the RDAS windows software, …
*** Raw data ***
   Time  Acc  Prs  Dig ADC0 ADC1 ADC2 ADC3 ADC4 ADC5
```

28 are recorded on board at 200 Hz, and 26 at 10 Hz, whose configuration names no trigger and no
firmware: by their names, the course's telemetry records of the same flights. The *R-DAS User
Manual* (V1.5, AED Electronics, 2001) says the raw export's header summarizes the configuration
and its values are the converters' counts, 5 V / 1,024 a step, to be converted with calibration
values set in the software (p. 14); that the R-DAS keeps the 2 s before liftoff (p. 6); that the
digital byte holds four digital inputs and, in its other four bits, the flight's phase (p. 14),
with a bit set when the timed recovery was used (p. 22); and that its sensors are an ADXL150
accelerometer and an MPXS4100A barometer (p. 7). It doesn't show the file, name its columns,
give the digital byte's bits, or give the counts' conversion; the course's own pressure lab names
another barometer on its R-DAS.

**Decision.**

1. **Two increments.** M7.1c8 reads the R-DAS's raw text. Its *done when*: every R-DAS raw
   text file in refs reads (`cargo xtask logs --check` fails on one that doesn't); the clock is
   checked against the configuration's stated rate, and a file whose average step is off it by
   more than 1%, or that states no rate, is refused (tests); every count is kept as written and
   `Dig` as its hexadecimal word's number; a configuration whose inputs aren't the table's
   columns is refused (tests); and the docs say what the manual gives and what is left out.
   M7.1c9 keeps M7.1c8's old entry and *done when*, the parent's, whole.
2. **No count is converted.** The file states no calibration value and doesn't say which board
   or sensors wrote it; the manual leaves the conversion to values held in the software, and
   sensors other than the manual's have been fitted. A conversion through a data sheet would be
   a number the file doesn't support, so `Acc`, `Prs` and `ADC0`–`ADC5` are
   `Quantity::Unrecognized`, as written, and the record's primary track is empty, as the Raven's
   `.FIPa`'s is. The format's altitude resolution is the manual's barometer count, 0.09 kPa,
   7.49 m near sea level.
3. **The clock counts seconds by the stated rate.** The configuration's `Data is sampled at N
   Hz.` is required; the clock's average step over the file must be its interval within 1%, and
   the note gives both. Every sample file's step is the interval exactly, and every first row is
   at −2.000 s, the 2 s before liftoff the manual describes.
4. **`Dig` is kept, not read into events.** In every on-board sample file the word goes `0000`,
   `0010` within 0.25 s after the zero, `0020`, `0030`, which looks like the manual's four
   phases in order; but the bits' values are in no document, so the word is kept as its number
   (`Quantity::Number`), read as hexadecimal since the files write letters (`00B0`), and no
   event is taken from it.
5. **The configuration is held to the table.** The inputs line must be present, every input it
   names one the reader knows (`Acceleration`, `Pressure`, `the digital inputs`, `ADC0`–`ADC5`),
   and the header must name their columns in order; its lines are kept in a note, at most 40 of
   200 characters, and a firmware version is the record's firmware.

**Consequences.** `hpr-flightdata` gains the `rdas` module and `LogFormat::RdasText`; the survey
reads the family as M7.1c8's and labels the families not yet read M7.1c9, 155 files. The docs
that pointed to M7.1c8 for the other loggers' formats point to M7.1c9. Reading phases from
`Dig`, or converting the counts, waits for a document that gives them.
