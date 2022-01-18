# noto-sans-mono-bitmap

Pre-rasterized bitmap font from "Noto Sans Mono", an open font from Google. 
* Downloaded from: <https://fonts.google.com/noto/specimen/Noto+Sans+Mono>
* License: SIL Open Font License (OFL) <https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL>

# TL;DR
* ✅ `no_std`, zero allocations
* ✅ most important symbols, numbers, and letters as pre-rasterized bitmap
* ✅ Noto Sans Mono font as base
* ✅ different sizes and font weights (light, normal, bold)
* ✅ nice anti-aliasing/smoothing and better looking than legacy bitmap fonts
* ✅ relevant font sizes: 14, 16, 24, 32, and 64px (all as optional build time features)

# Use Case
If you develop a kernel, you usually don't want to use the FPU (i.e. only soft float),
because otherwise you need to 

If all Cargo features are available, this bitmap fonts supports `light`, `regular`,
and `bold`, but no `italic` style, because Noto Sans Mono doesn't have an italic
TTF file. The rasterization was done with the awesome [fontdue-Crate](https://crates.io/crates/fontdue).

By default, all sizes and font styles/weights are included via the cargo feature `all`.
This can be restricted by only using features such as `regular` and `size_14`. Anyhow, 
a test of mine showed, that including all features in a release build only increases the 
file size by a few dozen kilobytes.

The bitmap font includes the following unicode range:
- BASIC LATIN,
- LATIN 1 Supplement
- LATIN EXTENDED-A

This means unicode symbols from `0 .. 0x17f`, hence letters and
and symbols from a QWERTZ/QWERTY keyboard plus symbols such as
Ö, Ä, and Ü. Control characters are not included.