# ADR-123: Complex packing, and a whole GFS file (2026-09-30)

- **Status:** accepted
- **Summary:** M5.2d2: complex packing (5.2, 5.3) and product template 4.8 in `hpr_io::grib2`; a whole GFS file's 746,770,303 values against ecCodes by hand, eight of its messages committed with ecCodes' sums and samples, the cut repacked by ecCodes for CI; NOMADS parsing keys layers by both surfaces, skips statistics and wraps a global grid

**Context.** M5.2d2's *done when*: a whole GFS file's fields decode to ecCodes' values, and
`hpr weather gfs --from` writes its profile. The file is GFS's 0.25° file of the 00 UTC run of
2026-09-30 at hour 18, `gfs.t00z.pgrb2.0p25.f018` (550,166,372 bytes, SHA-256 `648fcc36…c1c62a`),
from NOAA's open data bucket on AWS (`noaa-gfs-bdp-pds`): the run the recorded NOMADS cut
(ADR-121 §7) was taken from. ecCodes' survey of it: 743 messages, 742 in template 5.3 (all
second-order differencing, 1 to 3 bytes per descriptor, 21 with missing values marked in the data,
34 with a bitmap) and one 5.0; 701 in product template 4.0 and 42 in 4.8 (statistics over an
interval, such as accumulated rain). No 5.2 or first-order 5.3.

**Decision.**

1. **Complex packing in `hpr_io::grib2`** (`grib2/complex.rs`), from WMO-No. 306 Vol. I.2's
   templates 5.2, 5.3, 7.2 and 7.3: group references, widths and lengths, each list padded to a
   byte; the last group's length as given; primary and secondary missing values (code table 5.5)
   by all ones and all ones less one, in the value or, for a group of width 0, in its reference;
   first- and second-order differences rebuilt over the values present only, the packed integers
   in the first one or two places unused; then `Y = (R + h · 2^E) / 10^D` as for simple packing.
   `Field::packing` becomes an enum, `Packing::Simple` or `Packing::Complex`. The values of a
   complex field can only be read in order, so `Field::value` reads the field up to the point,
   `Field::values_at` reads several points in one pass, and `Field::values` returns a `Result`.
2. **Bounded, and refused by name where decoders differ.** `parse` reads each group's width and
   length once, stopping as soon as the lengths pass the packed count; with lists of 0 bits
   every group is alike and is counted at once, so the work is bounded by the file's size (the
   code review built a 2 MB file of 0-bit lists claiming 2^24 groups a field, and one whose sums
   overflowed 64 bits). The lengths must add up to the packed count and the values fit the data;
   a group may be at most 32 bits wide, the lists' entries at most 32 bits, 1 to 8 bytes per
   descriptor, order 1 or 2, and at most as many groups as values. Refused by name: a group
   splitting method other than 1 (with 0, row by row, lengths have no meaning); no groups for a
   nonzero count (ecCodes reads it as the reference everywhere, issue ECC-2095; the regulation is
   silent); secondary missing values with 0-bit references (no bits for all ones less one); and a
   first value with its top bit set, which ecCodes reads unsigned and NCEP's g2clib as sign and
   magnitude. Primary missing values with 0-bit references are read as ecCodes reads them (every
   group of width 0 missing; its encoder writes a field with no values that way). The rebuilt
   integers are not bounded by the headers, so the scale factors must keep `i64`'s ends finite,
   and rebuilding refuses a difference that overflows 64 bits (only a broken file does) when read.
   Each field keeps a private copy of the packing it was checked against and decodes from it.
3. **Product template 4.8** is read, with one time range (more are refused): the statistic (code
   table 4.10), the interval's length and unit, and its end, in `Product::statistics`;
   `Product::template` says which. `hpr_net::nomads` leaves statistics out.
4. **NOMADS parsing of a whole file.** A field is keyed by its parameter, its first surface and,
   for a layer, its second: the whole file's humidity over sigma 0.44 to 1 and 0.44 to 0.72 share a
   first surface and were refused as a duplicate. A layer is not a level (a test found a layer on
   500 hPa counted as a second 500 hPa level). A latitude/longitude grid whose `ni` steps make
   360° joins its last column to its first (`Grid::circles_the_earth`), so a site between 359.75°
   and 0° reads.
5. **Checked, the whole file, by a script run outside CI.** `validation/oracles/grib2/whole_file.py
   compare` streams every value `cargo xtask grib2-values` decodes and compares it with ecCodes
   2.49.0's, and writes its record, with ecCodes' survey of the file, to
   `validation/oracles/grib2/gfs-whole-file.json`: all 746,770,303 values agree within 4.4e-16
   relative (ecCodes multiplies by an inexact `10^−D`, one or two roundings), and the 24,642,017
   points without a value are the same points. The file is not committed (550 MB); it decoded in
   5 s in a release build on a Mac.
6. **Checked in CI.** Eight whole messages cut unchanged from the file (`crates/hpr-io/tests/
   fixtures/gfs-messages.grib2`, 404,731 bytes): for 1-, 2- and 3-byte descriptors and template
   4.8, the smallest over 2,000 bytes, for a message of some size; the smallest with a bitmap, with
   missing values in the data, and of template 4.8; and the one 5.0 field. The
   committed reading (`whole_file.py cut`) is, per message, its identity, template 4.8's interval,
   its missing points, `math.fsum` of its values and of each value times its index plus one, and
   every 997th value: 8.3 million values can't be committed. The sums hold the decoder
   to 1.35e-15 of the terms' sizes; in every committed message one packing step (`2^E / 10^D`) is
   at least 2.3e4 times that, so a single value off by a step fails the plain sum; the weighted one
   also depends on where each value sits. Two mutations were tried by hand, on the decoder before
   the review's rework of `layout`: a second-order start off by one failed the plain sum, and a
   missing-value code off by one failed the count of points without a value. Besides, the recorded GFS cut repacked by ecCodes
   (`repack.py`: 5.2, first- and second-order 5.3 in turn, 24 bits, 1 to 4 bytes per descriptor)
   joins the NOMADS tests (every value against ecCodes, the profile at every level) and gives
   `hpr weather gfs --from` a complex-packed file; its profile is the cut's within 6.9e-8 (bound
   7e-8).
   Hand-built messages pin each rule: groups, both kinds of missing value, both orders, a bitmap,
   and each refusal.
7. **`hpr weather gfs --from` the whole file** writes a profile in 0.33 s (release, on a Mac) whose 22
   levels are the cut's within 1.04e-7 relative, with 13 more above 10 hPa, where the cut stops;
   the same six levels are left out below the ground. The cut is repacked by NOMADS' filter in
   fewer bits (9 for a 650 hPa wind where the file has 13): by ecCodes alone, the two files'
   values differ by up to 2.4e-6 relative at the cut's grid points (`gfs-whole-file.json`). A test
   runs this where `refs/gfs/` holds the file (bound 1.1e-7) and passes without it, so CI skips
   it. Only 0.25° files read: `--from` holds a file to the model's grid (ADR-122 §3).
8. **Fixtures.** NCEP's output is a U.S. government work (17 U.S.C. § 105). ecCodes is run as an
   outside decoder and encoder, never ported.

**Consequences.** M5.2d2 is met. GFS's whole 0.25° files read offline. Files in JPEG 2000 (5.40)
stay refused, by name, until M5.2d3. RAP's whole files were not surveyed. The whole-file check is
by hand; CI holds eight of its messages and the repacked cut.
