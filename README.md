# noto-sans-mono-bitmap (Rust library)

Provides pre-rasterized characters from the "Noto Sans Mono" font in different sizes and font
weights for multiple unicode ranges. This crate is `no_std` and needs no allocations or floating
point operations. Useful in kernels and bootloaders when only "soft-float" is available. Strictly
speaking, this crate is more than a basic bitmap font, because it encodes each pixel as a byte
and not as a bit, which results in a much nicer result on the screen.

* Original font files taken from: <https://fonts.google.com/noto/specimen/Noto+Sans+Mono>
* License: SIL Open Font License (OFL) <https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL>


## TL;DR
* ✅ `no_std`, zero allocations, no floating point operations
* ✅ most important symbols, numbers, and letters as pre-rasterized bitmap
* ✅ Noto Sans Mono font as base
* ✅ different sizes and font weights (light, normal, bold)
* ✅ nice anti-aliasing/smoothing and better looking than legacy bitmap fonts
* ✅ every pixel is encoded in a byte (0-255) and not a bit, which results in a much nicer result on the screen.
* ✅ relevant font sizes: 14, 16, 24, 32, and 64px (as optional build time features)
* ✅ zero dependencies

![Screenshot of the bitmap font.](screenshot_bitmap_font.png "Screenshot of the bitmap font.")

## Terminology: Is Bitmap Font The Right Term?
Legacy (8x8) bitmap fonts usually refer to a font where each symbol is encoded in 8 bytes. The ones in a byte
(`0b00110000`) means "pixel on" and the zeroes' means "pixel off". However, my bitmap font actually encodes the
intensity of each pixel as a byte from 0 to 255. Hence, this is less size efficient than legacy bitmap fonts,
but looks much better. I still use the term bitmap font, because that term is used and known when talking
about pre-rasterized fonts/font rendering in an early stage of the boot process.

## When To Use This Crate
If you develop a kernel, you usually don't want to use the FPU (i.e. only soft float),
because otherwise you need to save the floating point registers on every context switch,
which is expensive. Because nice font rendering of TTF fonts heavily relies on many
floating point operations, this is not optimal inside a kernel (noticeable performance penalties).
Furthermore, in my experience it was hard to get some of the popular font rasterization
crates to compile with CPU features "+soft-float" and "-sse" (at least on x86_64).

Legacy 8x8 bitmap fonts are ugly when printed to the screen. My crate can be seen as a nice
replacement with very nice anti-aliasing.

If you have a standard environment or support for floating point operations, you might want
to rasterize the font by yourself with the crate `fontdue` and some TTF fonts rather than
using my crate.

## Minimal Code Example
```rust
use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapHeight, FontWeight};

// Minimal example.
fn main() {
    let width = get_bitmap_width(FontWeight::Regular, BitmapHeight::Size16);
    println!(
        "Each char of the mono-spaced font will be {}px in width if the font \
         weight={:?} and the bitmap height={}",
        width,
        FontWeight::Regular,
        BitmapHeight::Size16.val()
    );
    let bitmap_char = get_bitmap('A', FontWeight::Regular, BitmapHeight::Size16).expect("unsupported char");
    println!("{:?}", bitmap_char);
    for (row_i, row) in bitmap_char.bitmap().iter().enumerate() {
        for (col_i, pixel) in row.iter().enumerate() {
            println!("[{:02}][{:02}]: {:03}", row_i, col_i, pixel);
        }
    }
}
```

## Cargo Build Time Features
If all Cargo features are available, this bitmap fonts supports `light`, `regular`,
and `bold`, but no `italic` style, because Noto Sans Mono doesn't have an italic
TTF file. The rasterization was done with the awesome [fontdue-Crate](https://crates.io/crates/fontdue).

By default, all sizes and font styles/weights are included via the cargo feature `all`.
This can be restricted by only using features such as `regular` and `size_14`. Anyhow,
a test of mine showed, that including all features in a release build only increases the
file size by a few dozen to a few hundred kilobytes. The Rust compiler is really smart
throwing out unused parts of the bitmap font, even if they are included as dependency.
Your binary will not be bloated by a few megabytes, according to my findings.

The bitmap font includes the following unicode range:
- BASIC LATIN,
- LATIN 1 Supplement
- LATIN EXTENDED-A

This means unicode symbols from `0 .. 0x17f`, hence letters and
symbols from a QWERTZ/QWERTY keyboard plus symbols such as
Ö, Ä, and Ü. Control characters are not included.

## Quick Demo
`$ cargo run --example show_chars_in_window`

## Build Prerequisites
Because the examples uses "minifb" as dependency, on Linux the package `libxkbcommon-dev` is required
to run them. This is not necessary, if you just use this crate as dependency.

## License
See LICENSE file in repository.

## MSRV
Rust stable 1.52.1.
