# v0.3.0 (2024-04-xx)
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

# v0.2.0 (2022-10-07)
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

# v0.1.6 (2022-09-01)
- add the "■" character (U+25A0; [Geometric Shapes](https://jrgraphix.net/r/Unicode/25A0-25FF))

# v0.1.5 (2022-01-20)
- clearer terminology and doc
- explains that "bitmap" font is not 100% correct (see section Terminology in README)

# v0.1.4 (2022-01-20)
- fixed doc

# v0.1.3 (2022-01-20)
- added 'no_std' category in Cargo.toml

# v0.1.2 (2022-01-19)
- minor documentation improvements

# v0.1.1 (2022-01-19)
- Rust edition 2018 instead of 2021 to target a broader audience
- API adjustments: return Option in main library function and remove panics

# v0.1.0 (2022-01-19)
- initial release
