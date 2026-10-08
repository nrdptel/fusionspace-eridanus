# ADR-124: JPEG 2000 packing, through `hayro-jpeg2000` (2026-09-30)

- **Status:** accepted
- **Summary:** M5.2d3: JPEG 2000 (5.40) in `hpr_io::grib2` through `hayro-jpeg2000` in strict mode, lossless, to 21 bits, NCEP's coding only, the rest refused by name; four public RAP messages against ecCodes in CI

**Context.** M5.2d3's *done when*: a GRIB2 file in JPEG 2000 packing (data representation template
5.40) decodes to ecCodes' values. Template 5.40 codes a field's integers as a greyscale image in a
JPEG 2000 codestream (ISO/IEC 15444-1), then unpacks them as simple packing does,
`Y = (R + X · 2^E) / 10^D`. Messages sampled by byte range from NOMADS's 00 UTC run of
2026-09-30 were in 5.40 in each of RAP's pressure-level files (grids 32, 130, 200, 236, 242, 243,
252; `wrfmsl` is 5.3) and in 5.3 in NAM's. A JPEG 2000 decoder (tier-1 arithmetic decoding, tier-2 packets, the wavelets) is
far more code than the packings before it.

**Decision.**

1. **`hayro-jpeg2000` 0.4.0 decodes the codestream.** It is pure Rust, `MIT OR Apache-2.0`, forbids
   `unsafe` in its own code, and builds for wasm32; with no default features (only `std`) it pulls
   in no other crate. Writing a decoder in house was judged a milestone of its own for no gain in
   what is checked, since the check is against ecCodes either way. The C OpenJPEG (through
   `jpeg2k`) was ruled out: it links C and does not build for wasm32.
2. **Lossless, to 21 bits, the rest refused by name.** The crate runs the reversible 5/3 wavelet
   in `f32`. Its inverse adds two neighbouring high-pass coefficients before each floor, and for
   `B`-bit samples those sums reach nearly `16 · 2^(B−1)` (the cascaded analysis filters bound a
   coefficient by about `8.2 · 2^(B−1)`), so every step is exact only while that stays below
   `2^24`: `B ≤ 21`. The first draft took 24; review probes (fields ecCodes encoded on the
   fixture's grid) came back off by one in 121 and 136 of 10,152 values at 24 bits, and in one
   value in one of six probes at 23, every one whole and in range; ten probes at 21 and 22 were
   exact. More bits (`bits per value in JPEG 2000`) and
   lossy coding (code table 5.40's 1) are refused. Every decoded sample must also be a whole
   number from 0 to `2^bits − 1`.
3. **NCEP's coding only, checked before decoding; strict mode.** The main header and each
   tile-part header are read by hand (ISO/IEC 15444-1, Annex A) when the file is parsed: one
   unsigned component, no subsampling, no image or tile offset, one tile, a side of at most
   60,000 (the crate's limit), the 5/3 transform in COD and COC, no quantization in QCD and QCC,
   default precincts, and no marker but these, COM, TLM, PLM and PLT. SIZ, COD, COC and SOT must
   be exactly as long as their fields (the crate reads them field by field, so a longer length
   would hide a segment from the check that the crate still reads). Anything else is refused by
   name, and the image must hold one sample per packed value at section 5's bits. Review found
   that without these a 272-byte message with subsampled 1×1 tiles made the crate multiply past
   `u32` (a panic in debug builds, an eager allocation of about 1.3e9 tiles in release). The crate
   decodes in strict mode: its lenient mode filled a cut-short codestream with its DC offset,
   whole numbers in range, which review reproduced; a test cuts the fixture's codestream at every
   length. A field of 0 bits has no codestream and is `R / 10^D` everywhere, by the regulation;
   ecCodes gave `R` for one with `D = 2` (review's probe), so that case is not checked against it.
4. **Decoded whole, on each read.** A JPEG 2000 image is decoded whole, so `Field::value` decodes it
   for one point; `values` and `values_at` read it once. Nothing is cached in `Field`, which
   borrows the file and stays `Clone` and cheap.
5. **Checked against ecCodes in CI, on public data.** Four whole messages cut by byte range from
   RAP's 00 UTC run of 2026-09-30 (a U.S. government work): 500 hPa temperature on grids 200
   (6 bits) and 130 (9 bits, 151,987 points), and cloud base and top heights on grid 200 with
   bitmaps (15 and 16 bits; the top marks 434 of 10,152 points). `whole_file.py cut-rap` writes
   ecCodes 2.49.0's reading: counts of points and missing points, two correctly rounded sums over
   every value, and every 13th value. `tests/grib2_gfs.rs` holds `hpr_io::grib2` to it as it holds
   the GFS messages. A damage test edits the codestream at random and asks for a refusal, never a
   panic (5,000 cases run once by hand; CI runs proptest's default 256).

**Consequences.** M5.2d3 is met, and with it M5.2d and M5.2. No whole RAP file was read: `hpr
weather rap --from` on one is untried. NCEP codes a field with a bitmap as one row of its packed
values, so such a field of more than 60,000 values is refused by name (`JPEG 2000 image side`); a
whole grid-130 file may hold some. A new dependency, `hayro-jpeg2000`, sits in `hpr-io`. Lossy
JPEG 2000 and fields over 21 bits stay refused; the four fixture fields have 6 to 16 bits.
