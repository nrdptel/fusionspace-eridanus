# Damaged files

See what happens when a file or a server's answer is damaged, how that is tested and what the tests
leave out.

A flight log pulled off a card can be cut short, and a download can stop halfway. This page covers
every reader of outside bytes in the project: the design, motor, weather and elevation files, each
flight-log format, and the answers of the weather, elevation and motor servers. It is for anyone who
wonders what a bad file does, and for anyone adding a reader.

> **How far to trust it.** Damaged input gets an error, or a value from what was left, never a crash
> or a hang. That is tested, not proven. The tests below try 384 damaged inputs on each reader, and
> a file damaged some other way could still find a fault. When a read gives a value, nothing here
> says the value is right.

## What a reader does with one

A reader stops with an error that says what it couldn't read. Some damage doesn't stop it: a log cut
in half often still holds a shorter flight, so it reads as that flight. Check a log's end before you
trust its landing, and its top before you trust its apogee.

## How it is tested

A test walks a list of the readers. Five crates read outside bytes: `hpr-io`, `hpr-flightdata`,
`hpr-motor`, `hpr-format` and `hpr-net`. A check fails if a public function named `read…` or
`parse…` in them is missing from the list. Another fails if a flight-log format has no row. Each
reader is called directly and given:

- a real sample cut at 64 places, spread evenly from its first byte;
- the same sample with one byte changed, at 64 places: all its bits at every other place, only the
  lowest at the rest, so a length field moves far and a digit stays a digit;
- 256 cases of arbitrary bytes, up to 4,096 of them, from a seed fixed for each reader.

The test fails on a panic, or on a read still running after 10 s. It reads on from what a reader
gives, as a caller would: a GRIB2 file's values, a GeoTIFF's heights, a design from a `.ork`.
A sample is a committed public file where one exists. Where none does, the test builds one and says
why, as for a server's answer too long to read a few hundred times.

The maintainers' private collections get a test of their own, since their files are older and odder
than any sample. `cargo xtask logs --check`, `cargo xtask ork-flights --library --check` and `cargo
xtask fixture-flights --check` read 8 cuts of each log and design, print the counts and fail on a
panic.

## An example

On 2026-10-31, `cargo xtask logs --check` cut each of the collection's 1,122 logs at 8 places, 8,976
reads in all. Of those, 3,921 still gave a log, 5,055 were refused, and none panicked or ran past
10 s. So 44% of the cuts read as something, which is why a read isn't a sign that a log is whole.

## What it leaves out

- Other damage isn't tried, apart from what arbitrary bytes happen to do: two bytes changed, a
  stretch of bytes moved, or a file built to attack a reader.
- Very long files aren't tested for time or memory, since the inputs stay small. A read that runs
  out of memory or stack ends the program. That fails the test, but isn't told apart from a panic.
- Deep in a design, cuts reach nothing. A `.ork` is a zip with its directory at its end, and its XML
  must close, so a cut of either is refused first. A changed byte in the XML reaches further.
- A private file past 10 s is printed and counted, not failed, since a long log can take that long
  in a debug build.
- The tests call the Rust readers on a desktop system. The command line, Python and WebAssembly call
  the same code, but aren't run on damaged input here.
- These tests look only for a panic or a hang. Whether a damaged file's value is close to the truth
  is another question, and for a cut log it often isn't.
