//! Provides pre-rasterized characters from the "Noto Sans Mono" font in different sizes and font
//! weights for multiple unicode ranges. This crate is `no_std` and needs no allocations or floating
//! point operations. Useful in kernels and bootloaders when only "soft-float" is available. Strictly
//! speaking, this crate is more than a basic bitmap font, because it encodes each pixel as a byte
//! and not as a bit, which results in a much nicer result on the screen.
//!
//! * Original font files taken from: <https://fonts.google.com/noto/specimen/Noto+Sans+Mono>
//! * License: SIL Open Font License (OFL) <https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL>
//!
//! ## TL;DR
//! * ✅ `no_std`, zero allocations, no floating point operations
//! * ✅ most important symbols, numbers, and letters as pre-rasterized constant. Unicode-ranges are selectable.
//! * ✅ Noto Sans Mono font as base
//! * ✅ different sizes and font weights (light, normal, bold)
//! * ✅ nice anti-aliasing/smoothing and better looking than legacy bitmap fonts
//! * ✅ every pixel is encoded in a byte (0-255) and not a bit, which results in a much nicer result on the screen.
//! * ✅ relevant font sizes, such as 14, 18, 22, and 32px (as optional build time features)
//! * ✅ zero dependencies
//! * ✅ All characters are aligned in their box/raster. If they are printed next to each other, the result looks nice.
//!
//! Please check [README.md](https://github.com/phip1611/noto-sans-mono-bitmap-rs/blob/main/README.md)
//! for more details.

// # THIS FILE IS AUTO GENERATED BY THE PROJECT IN "../codegen" (see repository!)

#![no_std]
#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::suboptimal_flops,
    clippy::redundant_pub_crate,
    clippy::fallible_impl_from,
    clippy::large_stack_frames
)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]

// # THIS FILE IS AUTO GENERATED BY THE PROJECT IN "../codegen" (see repository!)

mod bold;
mod light;
mod regular;

/// Describes the relevant information for a rendered char of the font.
#[derive(Debug)]
pub struct RasterizedChar {
    /// The actual font data that is `height` * `width` bytes in size.
    /// Each byte describes the intensity of a pixel from 0 to 255.
    raster: &'static [&'static [u8]],
    /// Height of the raster box. The actual font size is slightly smaller.
    height: usize,
    /// The width of the rasterized char. It is guaranteed, that all chars
    /// of the same font weight and raster height also have the same width
    /// (as you would expect from a mono font.)
    width: usize,
}

impl RasterizedChar {
    /// The actual font data that is `height` * `width` bytes in size.
    /// Each byte describes the intensity of a pixel from 0 to 255.
    #[inline]
    pub const fn raster(&self) -> &'static [&'static [u8]] {
        self.raster
    }

    /// Height of the raster box. The actual font size is slightly smaller.
    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }

    /// The width of the raster char. It is guaranteed, that all chars
    /// of the same font weight and raster height also have the same width
    /// (as you would expect from a mono font).
    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }
}

/// Supported font weights.
///
/// The available variants depend on the selected Cargo build features.
#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum FontWeight {
    #[cfg(feature = "light")]
    Light,
    #[cfg(feature = "regular")]
    Regular,
    #[cfg(feature = "bold")]
    Bold,
}

impl FontWeight {
    /// Returns the numeric value of the enum variant.
    #[inline]
    pub const fn val(self) -> usize {
        self as _
    }
}

/// The height of the pre-rasterized font.
///
/// The font size will be a few percent less, because each letter contains
/// vertical padding for proper alignment of chars (i.e. ÄyA). The width of
/// each character will be also less than the height, because there is no
/// horizontal padding included.
///
/// The available variants depend on the selected Cargo build features.
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum RasterHeight {
    #[cfg(feature = "size_16")]
    Size16 = 16,
    #[cfg(feature = "size_20")]
    Size20 = 20,
    #[cfg(feature = "size_24")]
    Size24 = 24,
    #[cfg(feature = "size_32")]
    Size32 = 32,
}

impl RasterHeight {
    /// Returns the numeric value of the variant.
    #[inline]
    pub const fn val(self) -> usize {
        self as _
    }
}

/// Returns a [`RasterizedChar`] for the given char, [`FontWeight`], and [`RasterHeight`].
///
/// Returns None, if the given char is not known by the font. In this case,
/// you could fall back to `get_raster(' ', ...)`.
#[inline]
pub fn get_raster(c: char, style: FontWeight, size: RasterHeight) -> Option<RasterizedChar> {
    let raster = match style {
        #[cfg(feature = "light")]
        FontWeight::Light => match size {
            #[cfg(feature = "size_16")]
            RasterHeight::Size16 => crate::light::size_16::get_char(c),
            #[cfg(feature = "size_20")]
            RasterHeight::Size20 => crate::light::size_20::get_char(c),
            #[cfg(feature = "size_24")]
            RasterHeight::Size24 => crate::light::size_24::get_char(c),
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::light::size_32::get_char(c),
        },
        #[cfg(feature = "regular")]
        FontWeight::Regular => match size {
            #[cfg(feature = "size_16")]
            RasterHeight::Size16 => crate::regular::size_16::get_char(c),
            #[cfg(feature = "size_20")]
            RasterHeight::Size20 => crate::regular::size_20::get_char(c),
            #[cfg(feature = "size_24")]
            RasterHeight::Size24 => crate::regular::size_24::get_char(c),
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::regular::size_32::get_char(c),
        },
        #[cfg(feature = "bold")]
        FontWeight::Bold => match size {
            #[cfg(feature = "size_16")]
            RasterHeight::Size16 => crate::bold::size_16::get_char(c),
            #[cfg(feature = "size_20")]
            RasterHeight::Size20 => crate::bold::size_20::get_char(c),
            #[cfg(feature = "size_24")]
            RasterHeight::Size24 => crate::bold::size_24::get_char(c),
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::bold::size_32::get_char(c),
        },
    };

    raster.map(|raster| RasterizedChar {
        raster,
        height: size.val(),
        width: get_raster_width(style, size),
    })
}

/// Returns the width in pixels a char will occupy on the screen.
///
/// The width is constant for all characters regarding the same combination
/// of [`FontWeight`] and [`RasterHeight`]. The width is a few percent smaller
/// than the height of each char
#[inline]
pub const fn get_raster_width(style: FontWeight, size: RasterHeight) -> usize {
    match style {
        #[cfg(feature = "light")]
        FontWeight::Light => match size {
            #[cfg(feature = "size_16")]
            RasterHeight::Size16 => crate::light::size_16::RASTER_WIDTH,
            #[cfg(feature = "size_20")]
            RasterHeight::Size20 => crate::light::size_20::RASTER_WIDTH,
            #[cfg(feature = "size_24")]
            RasterHeight::Size24 => crate::light::size_24::RASTER_WIDTH,
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::light::size_32::RASTER_WIDTH,
        },
        #[cfg(feature = "regular")]
        FontWeight::Regular => match size {
            #[cfg(feature = "size_16")]
            RasterHeight::Size16 => crate::regular::size_16::RASTER_WIDTH,
            #[cfg(feature = "size_20")]
            RasterHeight::Size20 => crate::regular::size_20::RASTER_WIDTH,
            #[cfg(feature = "size_24")]
            RasterHeight::Size24 => crate::regular::size_24::RASTER_WIDTH,
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::regular::size_32::RASTER_WIDTH,
        },
        #[cfg(feature = "bold")]
        FontWeight::Bold => match size {
            #[cfg(feature = "size_16")]
            RasterHeight::Size16 => crate::bold::size_16::RASTER_WIDTH,
            #[cfg(feature = "size_20")]
            RasterHeight::Size20 => crate::bold::size_20::RASTER_WIDTH,
            #[cfg(feature = "size_24")]
            RasterHeight::Size24 => crate::bold::size_24::RASTER_WIDTH,
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::bold::size_32::RASTER_WIDTH,
        },
    }
}

// # THIS FILE IS AUTO GENERATED BY THE PROJECT IN "../codegen" (see repository!)
