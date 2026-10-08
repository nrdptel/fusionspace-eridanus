# ADR-128: M5.3c2, a site's height from a user's GeoTIFF, held to rasterio's reading (2026-09-30)

- **Status:** accepted
- **Summary:** M5.3c2: a user's GeoTIFF in `hpr_io::geotiff` over the `tiff` crate; geographic CRSs within a few metres of WGS 84 only, projections and far datums refused; the containing pixel placed as GDAL places it; GDAL's scale and offset, units the file states, else metres; held to rasterio 1.5.2 on seven fixtures and a whole USGS tile

**Context.** M5.3c2 asks for a site's height from an elevation file the user gives, matched
against another reader's. Elevation files (DEMs) mostly come as GeoTIFF: a TIFF image of heights
whose tags tie pixels to places (OGC GeoTIFF Standard 1.1, OGC 19-008r4, 2019). The USGS's 3D
Elevation Program (public domain), Copernicus and SRTM publish GeoTIFFs on a latitude and
longitude grid; the USGS's 1 m lidar comes projected (UTM). GDAL is the reader nearly every other
program uses, and rasterio 1.5.2 (BSD-3-Clause) wraps it, its wheel bundling GDAL 3.12.2. GDAL
reads some files differently from the standard (`frmts/gtiff/gtiffdataset_read.cpp`, MIT, read
at v3.12.2): it takes a negative `ScaleY` as north-up, prefers the pixel scale when a matrix is
also present, and turns `S_z` and the tiepoint's heights into a band scale and offset when the
file has a vertical CRS, as it does the `scale` and `offset` items of its `GDAL_METADATA` tag.
The TIFF layer (codecs, predictors, tiles, strips, BigTIFF, byte orders) is large; image-rs's
`tiff` 0.11.3 (MIT, 125 million downloads, 2026-02 release) decodes it in pure Rust, its
floating-point predictor included.

**Decision.**

1. **`hpr_io::geotiff`, over the `tiff` crate.** Its LZW and Deflate codecs only (PackBits and
   no compression are built in); JPEG, fax, WebP and zstd stay off, so no C code and nothing that
   stops wasm32. A file using another codec is refused at `parse`, naming it. The GeoKeys, the
   georeferencing and GDAL's tags are read here.
2. **Geographic CRSs near WGS 84 only.** `GTModelTypeGeoKey` 2, or a geodetic CRS key with no
   model type (GeoTIFF 1.0 writers), in degrees from Greenwich, and an EPSG code in
   `NEAR_WGS84`: WGS 84, NAD83 and its realisations, ETRS89, GDA94, GDA2020, NZGD2000, JGD2011,
   SIRGAS 2000, CGCS2000, all within a few metres of WGS 84 (plate motion since each was fixed),
   except near the rupture of a large earthquake since (several metres: Chile 2010 for SIRGAS
   2000, Wenchuan 2008 for CGCS2000, for example). JGD2000 is out: Japan's 2011 earthquake moved its
   north-east more than 5 m, and JGD2011 replaced it. A point's WGS 84 coordinates are read in the file's datum unchanged. Anything else (a
   projection such as UTM, NAD27 at about 55 m in New Mexico, Tokyo at hundreds) is refused naming
   its code, with the `gdalwarp -t_srs EPSG:4326` that converts it; datum shifts and projections
   would be models of their own, for later if users ask.
3. **The containing pixel, as GDAL places it.** A point reads the pixel whose area holds it, with
   no interpolation: GDAL's and rasterio's point sampling. The corner is GDAL's arithmetic
   (`X − I·S_x`, `Y − J·(−S_y)`, half a pixel back for pixel is point), so corners and pixel sizes
   come out bit for bit as GDAL's; the column is `⌊(λ − λ₀)/Δλ⌋`, the floor of a correctly rounded
   quotient. rasterio's `index` inverts the transform instead, so a point within about 1e-13 of a
   pixel of an edge can land on the other side of it; the tests sample random points, none on an
   edge. A longitude is tried as given and 360° to either side. One tiepoint with a pixel scale,
   or an unrotated matrix (its terms `a, d, f, h`). Refused, where GDAL and the standard differ or
   GDAL reads more: a negative `ScaleY`, a pixel scale beside a matrix (the standard forbids it),
   rotation, ground control points, and an internal mask image (GDAL's nodata mask).
4. **Heights: GDAL's scale and offset, then the file's unit.** A raw value `v` is
   `(v·scale + offset)·unit` metres. The scale and offset are GDAL's: `S_z` and `Z₀ − z₀·S_z` when
   any of the three is non-zero and GDAL reads a vertical CRS, else `GDAL_METADATA`'s first-band
   items, else 1 and 0; both sources disagreeing, or a scale of 0, are refused. Whether GDAL reads
   a vertical CRS turns on the directory's revision and on how GDAL and PROJ resolve the keys, so
   `S_z` applies only where that is certain: a GeoTIFF 1.1 directory of model type 2 naming a
   vertical CRS from the short list of EPSG codes below, with no `VerticalDatumGeoKey` and a
   geographic CRS other than WGS 84 3D (GDAL's `gt_wkt_srs.cpp`, with model type 2, drops the
   vertical CRS for a private key value and for datum 6030 beside WGS 84, and the whole CRS
   beside WGS 84 3D; with no model type it builds a local CRS, with a vertical part only when
   there is a unit key). It is ignored, as GDAL ignores it, in a 1.0 directory (rasterio shows
   GDAL dropping the vertical CRS there) and where no vertical key is present. Otherwise a file with
   heights in its tags is refused, unless they give GDAL's own scale 1 and offset 0 and
   `GDAL_METADATA` gives no other scale. GDAL matches `GDAL_METADATA` with quirks
   (`gtiffdataset_read.cpp`: attribute names in any case, compared with their prefix; C's `atoi`
   for the sample; text only as an item's one child, CDATA a child of its own; only ASCII blanks
   skipped). An item with a scale, offset or `unittype` role is therefore refused if it has a
   namespace, a capital in an attribute's name, a sample that isn't plain digits, a value that
   isn't a single text node, or the `IMAGE_STRUCTURE` domain; element names in either case and
   other domains read as GDAL reads them, and the skips GDAL makes (no name, no sample, another
   band, another root) were measured through rasterio and are made.
   The unit is `VerticalUnitsGeoKey` (metres, feet, US survey feet; refused if it disagrees with
   the vertical CRS's, which GDAL takes instead with model type 2), else the unit of a vertical CRS from a short
   list of EPSG codes (`VERTICAL_CRS_UNITS`: EGM2008, EGM96, EGM84, ODN, MSL, NAVD88 in metres, feet
   and US survey feet), else `GDAL_METADATA`'s `unittype` (refused if it disagrees with the keys,
   or names another unit; read after trimming ASCII blanks, of which GDAL drops only leading ones
   typed as they are); a file
   naming none is read as metres, flagged by `vertical_unit_stated` (GDAL reports no unit, except
   beside a vertical datum key alone, where it assumes metres too). A vertical CRS off the list is
   refused, with or without a unit key: with model type 2 GDAL takes its unit from EPSG's
   registry, which hpr doesn't hold. A user-defined one (32767, reported as no code) is read with a unit key and
   refused without. Vertical keys GDAL drops, unit and all, or reads by rules of its own, are
   refused (with model type 2, hpr would read a unit GDAL doesn't report; with none, GDAL reads
   the unit key alone and the refusal is conservative): a private value (above 32767) in any of
   them (dropped with a model type, read without one), any beside WGS 84 3D, datum 6030 beside
   WGS 84 with model type 2 (GDAL makes it WGS 84 3D), and any with no model type and no unit
   key. A blank `GDAL_METADATA` value written as a character reference, which GDAL reads as 0 or
   a blank unit, is refused.
   The vertical datum is reported, not applied. Nodata is `GDAL_NODATA` rounded to the
   sample type (a value an integer can't hold matches nothing); NaN is no data too.
5. **Bounded on a hostile file.** `height_at` decodes only the tile or strip holding the point;
   `parse` refuses a tile or strip over 256 MiB decoded (`MAX_CHUNK_BYTES`: the `tiff` crate's own
   chunk limit, which its padding of a floating-point tile bypasses) and a chunk count past a u32.
   The tile size is multiplied in u128, as a u64 product can overflow. `values` grows its output
   fallibly a row of tiles at a time, as they decode, up to 2²⁸ pixels. `GDAL_METADATA` nested
   past 16 deep is refused before its XML is parsed.
   `values_at` decodes each needed chunk once and fails only the points in a tile that won't
   decode. The `tiff` crate prints one `dbg!` line to standard error when a tag's value passes its
   1 MiB limit; that is its code, left as it is.
6. **The oracle.** `validation/oracles/geotiff/dem.py` cuts 70 by 50 pixels around Spaceport
   America from USGS 3DEP's tile n33w107 (pinned in `refs.lock.toml`) into seven encodings: float32
   LZW with the floating-point predictor on 16-pixel tiles; int16 Deflate with the horizontal
   predictor in 7-row big-endian strips; float64 uncompressed BigTIFF, pixel is point; uint16
   PackBits in NAVD88 US survey feet with nodata holes; uint32 centimetres with a scale and offset
   in `S_z` and EGM2008; uint8 quarter feet with a scale, offset and unit in `GDAL_METADATA`; int32 LZW on 32-pixel
   tiles with longitudes past 180°. It records rasterio's reading: the transform, the CRS, the
   vertical CRS and unit, the scale, offset and band unit, the nodata value, `math.fsum` of every value and
   of each value times its index, and 400 seeded points per file (the site, the rest random, some
   off the raster) with rasterio's pixel and value, each value checked against rasterio's own
   `sample`. The whole tile gets the same with 2,000 points. `tests/geotiff_rasterio.rs` holds hpr
   to all of it exactly, and every 97th point's height to rasterio's scale and offset applied to
   its value; the whole tile runs where `refs/` has it.

**Consequences.** M5.3c2 is met: all seven fixtures and the whole 3,612 by 3,612 tile read to
rasterio's corners, pixel sizes, scales and offsets bit for bit, to its two sums exactly (13
million values in the tile), and at 4,800 points (4,064 on a raster, 9 of them nodata) to its
pixel and value. Removing the pixel-is-point shift fails the float64 fixture's corner. At
Spaceport America (32.99° N, 106.97° W) the tile reads 1,400.691 m, which the USGS states is
above NAVD88; Open-Meteo's answer there (ADR-126) is 1,400 m. M5.3 is complete. `hpr` has no
command for a site's height yet, and no flight reads one from a file; a projected file, or one on
an older datum, needs `gdalwarp` first. Review found the first draft's gaps: a tile header that
could ask for 32 GiB, GDAL's scale and offset unread (a decimetre file read 10× high), any datum
accepted. A second round found a tile size that overflowed a u64, `S_z` applied where GDAL
would not, JGD2000 in the near list and unbounded XML nesting; a third, GDAL's metadata matched
more narrowly than GDAL matches it, and a unit key read where GDAL takes the vertical CRS's; a
fourth and fifth, gaps of the same kinds (metadata quirks, an unlisted vertical CRS's unit,
GDAL's own `S_z` 1 beside another metadata scale, vertical keys GDAL drops), answered by failing
closed. All are fixed and tested
above.
