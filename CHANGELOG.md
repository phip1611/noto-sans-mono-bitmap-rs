# Changelog

## v0.4.0 (TBD)
- Rust edition 2024
- MSRV 1.85.0
- Improved rendering of all characters:
  - The raster height is now the line height of the font (ascender and
    descender). Hence, the characters are about 7% larger than before and use
    the available space better while none of them is cut off.
  - All characters share a common baseline that is aligned to the pixel grid.
  - The outlines are auto-hinted (vertical direction only), which makes the
    baseline, x-height, cap height, and horizontal stems crisp.
  - Characters are placed according to the font's horizontal metrics.
  - Wide characters, such as `�`, are scaled down to fit into their raster
    instead of being cut off. Characters that are only slightly too wide (a
    few bold ones, such as `ď`) are compressed horizontally. Previously, the
    cut-off parts were smeared into the first column of the raster.
  - The raster widths are unchanged.
- **Breaking** `RasterizedChar` now stores its pixels flat in row-major order
  (`width * height` bytes, top row first) instead of as one slice per row
  (`&[&[u8]]`). `RasterizedChar::raster()` was replaced by:
  - `pixels() -> &'static [u8]`: all pixels, flat and row-major; the pixel at
    `(x, y)` is `pixels()[y * width() + x]`
  - `rows() -> core::slice::ChunksExact<'static, u8>`: iterator over the rows
  - `pixel(x, y) -> Option<u8>` (`const`): a single pixel

  `RasterizedChar` now also implements `Clone`, `Copy`, `PartialEq`, and `Eq`.

  **Migration:** replace `raster.raster().iter()` with `raster.rows()`, and
  `raster.raster()[y][x]` with `raster.pixel(x, y).unwrap()` or
  `raster.pixels()[y * raster.width() + x]`.
- The glyphs are now stored as one binary blob per font weight, raster height,
  and unicode range (included via `include_bytes!`) instead of thousands of
  generated Rust source files. This shrinks the crate, speeds up compilation,
  and removes the per-row pointers (and their relocations in position
  independent binaries) from the compiled binary. `get_raster()` and
  `get_raster_width()` remain `const fn`.

## v0.3.2 (2026-01-07)
- `fn get_raster()` is not `const`

## v0.3.1 (2024-12-17)
- Fixed typo in documentation

## v0.3.0 (2024-04-02)
- Improved alignment of characters in a line. As a consequence, they are now
  slightly smaller. The correct line height to font size ratio is about 0.75
  rather than the previous 0.84. A lot of thanks to
  [toothbrush7777777](https://github.com/toothbrush7777777) for their
  contribution!
- The replacement character `�` is cut off on the left and right. There is no
  better solution at this point. However, other programs, such as Jetbrains
  IDEs also cut it off in some scenarios (depending on font and other
  properties), so this should be okay. (I'm not 100% sure yet, but I think this
  letter is not part of the noto-sans-mono font, so we only can get it from the
  font rendering lib, which uses different metrics.)
- Some internal code improvements

## v0.2.0 (2022-10-07)
- **Breaking** renamed `get_bitmap` to `get_raster`
- **Breaking** renamed `get_bitmap_width` to `get_raster_width`
- **Breaking** renamed `BitmapHeight` to `RasterHeight`
- **Breaking** Now there are only the following `RasterHeight` available: 16, 20, 24, 32.
  Otherwise the space requirements are too big, especially, if new symbols are added in the future.
- it's clear now which unicode ranges are supported:
  check the `Cargo.toml`'s feature section
- changed the amount and naming of modules that are offered
  - now, there is a `*_default` and `*_all` module for font-weight, raster height,
    and unicode ranges

Furthermore, I investigated the impact on size. The crate size (what needs to be downloaded) is
relatively high. It is multiple MiB. However, after compilation, this is very small. The compiler
reliably discards unused code paths. For example, if `basic-latin` and `latin-1-supplement` are
used with `FontWeight::Regular` and `RasterHeight::14`, then the overhead is less than 200KiB.
If the full support of all currently supported font weights and raster heights is needed, this adds
about 5 MiB to the binary.

While the compiler can reliable discards unused font weights, it can not reliably discard unused
unicode ranges. Usually, if you pass a `char` to `get_raster`, the whole variety of unicode ranges
can be reached. Thus, Rust doesn't discard unused ranges. Hence, it is recommended to only select
the unicode ranges that you do need.

## v0.1.6 (2022-09-01)
- add the "■" character (U+25A0; [Geometric Shapes](https://jrgraphix.net/r/Unicode/25A0-25FF))

## v0.1.5 (2022-01-20)
- clearer terminology and doc
- explains that "bitmap" font is not 100% correct (see section Terminology in README)

## v0.1.4 (2022-01-20)
- fixed doc

## v0.1.3 (2022-01-20)
- added 'no_std' category in Cargo.toml

## v0.1.2 (2022-01-19)
- minor documentation improvements

## v0.1.1 (2022-01-19)
- Rust edition 2018 instead of 2021 to target a broader audience
- API adjustments: return Option in main library function and remove panics

## v0.1.0 (2022-01-19)
- initial release
