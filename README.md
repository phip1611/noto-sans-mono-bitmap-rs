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
* ✅ most important symbols, numbers, and letters as pre-rasterized constant. Unicode-ranges are selectable.
* ✅ Noto Sans Mono font as base
* ✅ different sizes and font weights (light, normal, bold)
* ✅ nice anti-aliasing/smoothing and better looking than legacy bitmap fonts
* ✅ every pixel is encoded in a byte (0-255) and not a bit, which results in a much nicer result on the screen.
* ✅ relevant font sizes: 14, 16, 24, and 32px (as optional build time features)
* ✅ zero dependencies
* ✅ All characters are aligned in their box/raster. If they are printed next to each other, the result looks nice.

![Screenshot of the bitmap font.](screenshot_bitmap_font.png "Screenshot of the bitmap font.")

## Developer Guide and Contributing
Please check [CONTRIBUTING.md](./CONTRIBUTING.md).

## Terminology: Is Bitmap Font The Right Term?
Legacy (8x8) bitmap fonts usually refer to a font where each symbol is encoded in 8 bits. The ones in a byte
(`0b00110000`) means "pixel on" and the zeroes' means "pixel off". However, my font actually encodes the
intensity of each pixel as a byte from 0 to 255. Hence, this is less size efficient than legacy bitmap fonts,
but looks much better. I still use the term bitmap font, because that term is used and known when talking
about pre-rasterized fonts/font rendering in low-level contexts, such as the boot process.

## When (Not) To Use This Crate
If you want to print to a framebuffer and if you develop a bootloader or a kernel, you usually don't
want to enable the FPU and refrain from floating point instruction (i.e. only use soft float).
My crate is a good option to print characters nicely to the screen in such scenarios. As nice live
font rendering of TTF fonts heavily relies on many floating point operations, which is not optimal
inside a low level binary. Legacy 8x8 bitmap fonts are ugly when printed to the screen.
`noto_sans_mono_bitmap` can be seen as a nice replacement with very nice anti-aliasing.

If you have a standard environment or support for floating point operations, you might want
to rasterize the font by yourself with the crate `fontdue` and some TTF fonts rather than
using my crate.

## Minimal Code Example
```rust
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

// Minimal example.
fn main() {
    let width = get_raster_width(FontWeight::Regular, RasterHeight::Size16);
    println!(
        "Each char of the mono-spaced font will be {}px in width if the font \
         weight={:?} and the bitmap height={}",
        width,
        FontWeight::Regular,
        RasterHeight::Size16.val()
    );
    let char_raster =
        get_raster('A', FontWeight::Regular, RasterHeight::Size16).expect("unsupported char");
    println!("{:?}", char_raster);
    // The pixels are stored in row-major order: one byte (intensity 0..=255) per pixel.
    for (row_i, row) in char_raster.rows().enumerate() {
        for (col_i, pixel) in row.iter().enumerate() {
            println!("[{:02}][{:02}]: {:03}", row_i, col_i, pixel);
        }
    }
}
```

## Cargo Features and Crate Size
The `external/check-size` crate provides you with insights on the overhead of
having this library statically compiled into the binary.

Each glyph is stored as exactly `width * height` bytes (one byte per pixel)
without any per-glyph or per-row overhead, and the glyphs of each combination
of font weight, raster height, and unicode range are one contiguous blob. Only
the blobs of enabled features are compiled in.

By default, only a reasonable subset of possible features is included (size 16, regular font,
only ASCII), which adds roughly 10 KiB of glyph data to the binary. Even with the `all` feature,
depending on your code, the compiler can reliably discard unreachable or unused font weights and
raster heights. Nevertheless, it is recommended to include no more features than necessary.

With all features included inside the binary, and without any discarding by the compiler, you
can expect roughly 1 MiB of memory consumption. However, this would require the rather unlikely
case that you use different sizes and font weights simultaneously and all the unicode ranges.


## Quick Demo
`$ cargo run --example show_chars_in_window`

If you want to use special chars, such as `{ä, ö, ü, �}`, in the example,
you can use

`$ cargo run --example show_chars_in_window --features all`

## Limitations & FAQ
- _The replacement character `�` is smaller than other characters._ \
  In Noto Sans Mono, `�` is twice as wide as all other characters. To keep the
  mono font guarantee, it is scaled down to fit into a single character cell.
- _How are the characters placed in their raster?_ \
  The raster height is the line height of the font (ascender and descender),
  so all characters share a common baseline and none of them is cut off. You
  can render lines directly below each other without additional spacing. The
  raster width is the advance width of the mono font. The outlines are
  grid-fitted with an auto-hinter (vertical direction only), which makes small
  sizes crisper. The pixel values describe the coverage of the pixel by the
  glyph (no gamma correction applied).

## Trivia
The glyphs are scaled and auto-hinted with [skrifa](https://crates.io/crates/skrifa)
and rasterized with [ab_glyph_rasterizer](https://crates.io/crates/ab_glyph_rasterizer).
Thanks to the original author(s)!

## License
See LICENSE file in repository.

## MSRV (for library consumers)
Rust stable 1.85.0.
