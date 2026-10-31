# Damaged files

See what happens when a file or a server's answer is damaged, how that is tested and what the tests
leave out.

A flight log pulled off a card can be truncated or corrupt, and a download can stop halfway. This page covers
every reader in the five crates that read files and servers' answers. They read the design, motor,
weather and elevation files, each flight-log format, and what the weather, elevation and motor
servers send. It is for anyone who wonders what a bad file does, and for anyone adding a reader.

> **How far to trust it.** On the damaged inputs tried, a reader gave an error, or a value from what
> was left, and never crashed or hung. That is tested, not proven. The tests try 384 small inputs on
> each entry of their list. Very large files and other damage aren't tried. When a read gives a
> value, nothing here says the value is right.

## What a reader does with one

A reader stops with an error that says what it couldn't read. If that happens, copy the file off the
card again, or export it again from the logger's own software.

Some damage doesn't stop it: a log cut in half often still holds a shorter flight, so it reads as
that flight. Check a log's end before you trust its landing, and its top before you trust its apogee.
Compare them with what the logger reported on the pad.

## How it is tested

A test walks a list of the readers in `hpr-io`, `hpr-flightdata`, `hpr-motor`, `hpr-format` and
`hpr-net`, the [crates](../glossary.md#crate) that read outside files. Each entry on the list is a
reader and a sample for it. A check fails if a public function named `read…` or `parse…` in them has
no entry. Another fails if a flight-log format has none. Each reader is called directly, or through
the parser that feeds it if it takes a parsed value. Each is given:

- a real sample cut at 64 places, spread evenly from its first byte;
- the same sample with one byte changed, at 64 places: all its bits at every other place, only the
  lowest at the rest;
- 256 cases of arbitrary bytes, up to 4,096 of them, from a [seed](../glossary.md#seed) fixed for each entry.

Changing all of a byte's bits moves a length field far. Changing the lowest keeps a digit a digit.

The test fails on a panic, the program's own crash on a broken assumption, or on a read still running
after 10 s. Many entries read on from what a
reader gives, as a caller would: a GRIB2 file's values, a GeoTIFF's heights, a design from a `.ork`.
A sample is a committed public file where one serves. Where none does, the test builds one and says
why, as for a server's answer too long to read a few hundred times.

The maintainers' private collections, larger than the committed samples, get a test of their own.
`cargo xtask logs --check`, `cargo xtask ork-flights --library --check` and `cargo xtask
fixture-flights --check` read 8 cuts of each log and each design in them, and of a design's XML. They
print the counts and fail on a panic.

## An example

On October 31, 2026, the maintainers' `cargo xtask logs --check` cut each of the private collection's
1,122 sample files at 8 places (they aren't in the repository, so you can't rerun this), 8,976
reads in all. Of those, 3,921 still gave a log, 5,055 were refused, and none panicked or ran past
10 s. So 44% of the cuts read as something, which is why a read isn't a sign that a log is whole.

## What it leaves out

- Other damage isn't tried: two bytes changed, a stretch of bytes moved, or a file built to attack a
  reader. Arbitrary bytes might do any of these, by chance.
- Very long files aren't tested for time or memory in the public tests, whose inputs stay small. A
  read that runs out of memory or stack ends the program. That fails the test and names no place.
- Other readers aren't on the list: a Mach table's CSV in `hpr-aero`, and the JSON the command line
  reads for a design or a sounding ([#1079](https://github.com/nrdptel/fusionspace-eridanus/issues/1079)).
- Deep in a design, cuts reach nothing. A `.ork` is a zip with its directory at its end, and its XML
  must close. So a cut of either is refused first. A changed byte in the XML reaches further. Other
  checked layers stop damage early too: a workbook's zip, a `.hprz`'s, and an RTx raw file's slots.
- A private file past 10 s is printed and counted, not failed, since a long log can take that long
  in an unoptimized build.
- The tests call the Rust readers on a desktop system. The command line, Python and WebAssembly call
  the same code, but aren't run on damaged input here.
- These tests look only for a panic or a hang. Whether a damaged file's value is close to the truth
  isn't measured, and it can be far off: a log cut before its top gives too low an apogee.
