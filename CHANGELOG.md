# v0.2.0 (2022-09-25)
- **Breaking** renamed `get_bitmap` to `get_raster`
- **Breaking** renamed `get_bitmap_width` to `get_raster_width`
- **Breaking** renamed `BitmapHeight` to `RasterHeight`
- it's clear which unicode ranges are supported:
  currently only `basic-latin` and `latin-1-supplement`
- changed the amount and naming of modules that are offered
  - now, there is a `*_default` and `*_all` module for font-weight, raster height,
    and unicode ranges

Apart from that, I investigated the impact on size. The crate size (what needs to be downladed) is
relatively high. It is multiple MiB. However, after compilation, this is very small. The compiler
reliably discards unused code paths. For example, if `basic-latin` and `latin-1-supplement` are
used with `FontWeight::Regular` and `RasterHeight::14`, then the overhead is less than 120KiB.
If the full support of all currently supported font weights and raster heights is needed, this adds
about 5 MiB to the binary.

While the compiler can reliable discards unused font weights, it can not reaily discard unused
unicode ranges. Usually, if you put a `char` in, the whole varitey of unicode ranges can be meaned.
Thus, Rust doesn't discard unused ranges. Thus, it is recommended to only use the unicode ranges
that you do need.

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
