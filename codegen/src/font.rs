//! Rasterizes the glyphs of the Noto Sans Mono font files into fixed-size cells.
//!
//! The generated crate ships these cells as pre-rasterized bytes, so that its
//! users (kernels, bootloaders) can print nice text without any font rendering
//! or floating point math at runtime. All font handling happens here, at build
//! time.
//!
//! # Terminology
//!
//! - **Glyph:** the visual shape of a character in a font, e.g. the shape of
//!   'A'. The font file stores it as an outline (lines and curves).
//! - **Cell:** the fixed-size box of pixels a glyph is drawn into (also called
//!   raster). All cells of one font weight and raster height have the same
//!   size, so text can be laid out on a grid, like in a terminal.
//! - **Baseline:** the line letters sit on. Descenders, as in 'g' or 'p', reach
//!   below it.
//! - **Ascender, descender:** how far the font reaches above and below the
//!   baseline, including room for accents and descenders.
//! - **Advance width:** how far the text cursor moves after a glyph. It is the
//!   same for all glyphs of a mono font.
//! - **Hinting:** adjusting an outline so that its edges line up with the pixel
//!   grid, which makes small text crisp.
//! - **Coverage:** how much of a pixel the glyph covers, from 0 (not at all) to
//!   255 (fully).
//!
//! # Who Does What
//!
//! - [`skrifa`] reads the font file: the font's metrics (ascender, advance
//!   width, ...), which glyph belongs to a character, and each glyph's outline
//!   as lines and curves. It scales the outline to pixels and grid-fits it with
//!   its auto-hinter.
//! - [`ab_glyph_rasterizer`] turns lines and curves into pixels: it computes
//!   how much of each pixel the outline covers, which gives smooth
//!   (anti-aliased) edges. It knows nothing about fonts.
//! - This module chooses the cell size and where each outline goes in its
//!   cell, and passes the outline from one crate to the other (`RasterPen`).
//!
//! # How It Works
//!
//! For each font weight and raster height, [`RasterizationInfo`]:
//! 1. Derives the cell from the font's metrics: the font size is chosen so that
//!    the line height (ascender and descender) fills the cell height, the
//!    baseline is snapped to a pixel row, and the cell width is the rounded
//!    advance width of the mono font.
//! 2. Gets the outline of each glyph from [`skrifa`], grid-fitted by its
//!    auto-hinter.
//! 3. Places the outline in the cell: regular glyphs sit on the common baseline
//!    with their advance width centered. Glyphs slightly wider than the cell
//!    (e.g. bold 'ď') are compressed horizontally; much wider ones (e.g. '�')
//!    are scaled down.
//! 4. Turns the placed outline into pixels with [`ab_glyph_rasterizer`]. Each
//!    byte is the coverage of a pixel (0: empty, 255: full), without gamma
//!    correction.
//!
//! # Aligning Glyphs in a Cell
//!
//! Text only looks right if the glyphs of neighboring cells line up. The
//! common practice, e.g. in terminal emulators, is to derive all positions
//! from the font's metrics rather than from the ink of each glyph:
//!
//! - **One common baseline:** the baseline is on the same row in every cell
//!   and each glyph is drawn relative to it. Centering the ink of each glyph
//!   vertically instead would move short letters like 'x' up and letters with
//!   descenders like 'g' even further up, so the text would look wavy.
//! - **Cell height from the font's line height:** the cell covers the
//!   ascender and descender, so accents ('Å') and descenders ('g') fit without
//!   being cut off, and lines can be stacked without gaps or overlaps.
//! - **Baseline on a pixel boundary:** otherwise, the bottom edge of every
//!   letter is blurred across two pixel rows.
//! - **Placement by advance width, not by ink:** each glyph keeps its position
//!   on its advance width, so the spacing the font designer intended is kept.
//! - **Shrink, don't clip:** a glyph that is too wide is compressed or scaled
//!   down, never cut off.
//!
//! # Why This Way
//!
//! - **Light auto-hinting** (like FreeType's `FT_LOAD_TARGET_LIGHT`): at the
//!   small font sizes used here (about 12 to 24 px), unhinted edges blur
//!   across two pixel rows. The hinter snaps the baseline, x-height, cap
//!   height, and horizontal stems to the pixel grid. It works vertically only,
//!   so Latin glyphs keep their designed shape and spacing (glyphs of unknown
//!   scripts, such as arrows, may also be hinted horizontally). The
//!   auto-hinter is forced, as the font files contain no meaningful hinting
//!   instructions.
//! - **Placing outlines, not pixels:** the vector outline is positioned before
//!   it is rasterized, which keeps edges exact at sub-pixel offsets.
//! - **Failing loudly:** a missing glyph or a glyph exceeding its cell panics,
//!   so that broken glyphs are never shipped.

use ab_glyph_rasterizer::{Point, Rasterizer, point};
use skrifa::instance::{LocationRef, Size};
use skrifa::metrics::BoundingBox;
use skrifa::outline::pen::ControlBoundsPen;
use skrifa::outline::{
    DrawSettings, Engine, HintingInstance, HintingOptions, OutlineGlyph, OutlinePen, SmoothMode,
    Target,
};
use skrifa::{FontRef, MetadataProvider};
use std::fmt::{Debug, Formatter};

const NOTO_SANS_MONO_REGULAR: &[u8] = include_bytes!("res/NotoSansMono-Regular.ttf");
const NOTO_SANS_MONO_BOLD: &[u8] = include_bytes!("res/NotoSansMono-Bold.ttf");
const NOTO_SANS_MONO_LIGHT: &[u8] = include_bytes!("res/NotoSansMono-Light.ttf");

/// Strongest horizontal compression applied to glyphs that are too wide.
///
/// Wider glyphs are scaled down uniformly instead. A slight compression is
/// invisible, whereas uniform scaling hints the glyph at a smaller size, which
/// may cost a pixel of height.
const MIN_HORIZONTAL_SQUEEZE: f32 = 0.8;

/// All available fonts. Must match the order in [`FontWeight`]!
const NOTO_SANS_FAMILY: [&[u8]; 3] = [
    // must match order in enum FontWeightName
    NOTO_SANS_MONO_LIGHT,
    NOTO_SANS_MONO_REGULAR,
    NOTO_SANS_MONO_BOLD,
];

pub const fn noto_font_by_weight(typ: &FontWeight) -> &'static [u8] {
    NOTO_SANS_FAMILY[typ.name.val()]
}

/// Font weights that the codegen process generates.
pub const SUPPORTED_FONT_WEIGHTS: &[FontWeight] = &[
    FontWeight::new(FontWeightName::Light, false),
    FontWeight::new(FontWeightName::Regular, true),
    FontWeight::new(FontWeightName::Bold, false),
];

/// Supported font weights for the code generation. Corresponds to the available TTF font files.
#[derive(Debug, Copy, Clone)]
pub struct FontWeight {
    name: FontWeightName,
    // if the feature is included by default in Cargo.toml
    default_feature: bool,
}

impl FontWeight {
    pub const fn new(name: FontWeightName, default_feature: bool) -> Self {
        Self {
            name,
            default_feature,
        }
    }

    pub const fn name(&self) -> &FontWeightName {
        &self.name
    }

    pub const fn mod_name(&self) -> &str {
        self.name.mod_name()
    }

    pub const fn default_feature(&self) -> bool {
        self.default_feature
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum FontWeightName {
    // must match order in array NOTO_SANS_FAMILY
    Light,
    Regular,
    Bold,
}

impl FontWeightName {
    /// Returns the numeric value of the enum variant.
    pub const fn val(self) -> usize {
        self as _
    }

    /// Returns a lowercase string describing the font weight.
    pub const fn mod_name(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Regular => "regular",
            Self::Bold => "bold",
        }
    }
}

/// Rasterizes the characters of a font into fixed-size cells, like a terminal.
///
/// One instance covers one font weight and raster height. See the
/// [module documentation](crate::font) for how and why.
pub struct RasterizationInfo<'a> {
    font: FontRef<'a>,
    /// Hinting instance for [`Self::font_size`].
    hinting: HintingInstance,
    /// Height of each cell in pixels.
    raster_height: usize,
    /// Width of each cell in pixels. The same for all glyphs (mono font).
    raster_width: usize,
    /// Font size in pixels per em, about 73% of the raster height.
    font_size: f32,
    /// Distance from the top of the cell to the baseline in pixels.
    baseline: f32,
    /// Advance width of the mono font in pixels.
    advance_width: f32,
}

impl Debug for RasterizationInfo<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RasterizationInfo")
            .field("raster_height", &self.raster_height)
            .field("raster_width", &self.raster_width)
            .field("font_size", &self.font_size)
            .field("baseline", &self.baseline)
            .field("advance_width", &self.advance_width)
            .finish_non_exhaustive()
    }
}

impl<'a> RasterizationInfo<'a> {
    /// Prepares the rasterization of a font into cells of the given height.
    ///
    /// `font_bytes` are the raw bytes of a TrueType/OpenType font file.
    pub fn new(raster_height: usize, font_bytes: &'a [u8]) -> Self {
        let font = FontRef::new(font_bytes).unwrap();

        let metrics = font.metrics(Size::unscaled(), LocationRef::default());
        let units_per_em = metrics.units_per_em as f32;
        // The descent is negative.
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let font_size = raster_height as f32 * units_per_em / line_height;
        let scale = font_size / units_per_em;
        // Center the line gap and snap the baseline to the pixel grid, so that
        // the bottom edge of the letters is crisp.
        let baseline = ((metrics.ascent + metrics.leading / 2.0) * scale).round();

        // Shared by all glyphs of the mono font, except a few such as '�'.
        let advance_width = font
            .glyph_metrics(Size::new(font_size), LocationRef::default())
            .advance_width(font.charmap().map('M').unwrap())
            .unwrap();

        Self {
            hinting: Self::hinting_instance(&font, font_size),
            font,
            raster_height,
            raster_width: advance_width.round() as usize,
            font_size,
            baseline,
            advance_width,
        }
    }

    /// Creates the hinter that snaps outlines to the pixel grid.
    ///
    /// The auto-hinter is forced, as the font files contain no meaningful
    /// hinting instructions.
    fn hinting_instance(font: &FontRef, font_size: f32) -> HintingInstance {
        let options = HintingOptions {
            engine: Engine::Auto(None),
            target: Target::Smooth {
                mode: SmoothMode::Light,
                symmetric_rendering: true,
                preserve_linear_metrics: true,
            },
        };
        HintingInstance::new(
            &font.outline_glyphs(),
            Size::new(font_size),
            LocationRef::default(),
            options,
        )
        .unwrap()
    }

    /// Rasterizes a character into a cell of its own.
    ///
    /// First, the glyph's vector outline is placed in the cell (and scaled if
    /// it doesn't fit). Then, `draw_into_cell` turns it into pixels. Placing the
    /// outline rather than finished pixels keeps edges exact at sub-pixel
    /// offsets.
    ///
    /// Panics if the font has no glyph for the character, so that the font's
    /// placeholder glyph is never shipped.
    pub fn rasterize(&self, c: char) -> Vec<Vec<u8>> {
        let glyph_id = self
            .font
            .charmap()
            .map(c)
            .unwrap_or_else(|| panic!("font has no glyph for {c:?}"));
        let glyph = self.font.outline_glyphs().get(glyph_id).unwrap();

        let Some(bounds) = glyph_bounds(&glyph, &self.hinting) else {
            // Whitespace
            return vec![vec![0; self.raster_width]; self.raster_height];
        };
        let scale_x = (self.raster_width as f32 / (bounds.x_max - bounds.x_min)).min(1.0);
        if scale_x < MIN_HORIZONTAL_SQUEEZE {
            self.draw_scaled_down(c, &glyph, bounds, scale_x)
        } else {
            self.draw_on_baseline(c, &glyph, bounds, scale_x)
        }
    }

    /// Draws a regular glyph on the common baseline.
    ///
    /// The advance width is centered in the cell. Glyphs slightly wider than
    /// the cell are compressed horizontally by `scale_x`, which keeps their
    /// hinting intact. Glyphs are never moved vertically, as this would break
    /// the common baseline.
    fn draw_on_baseline(
        &self,
        c: char,
        glyph: &OutlineGlyph,
        bounds: BoundingBox,
        scale_x: f32,
    ) -> Vec<Vec<u8>> {
        let raster_width = self.raster_width as f32;
        // Center the advance width, but keep the ink inside the cell.
        let dx = ((raster_width - self.advance_width) / 2.0)
            .max(-bounds.x_min * scale_x)
            .min(bounds.x_max.mul_add(-scale_x, raster_width));
        self.draw_into_cell(c, glyph, &self.hinting, scale_x, dx, self.baseline)
    }

    /// Draws a glyph that is much wider than the cell (e.g. '�') scaled down.
    ///
    /// The glyph is hinted at the smaller font size and centered horizontally.
    /// Its vertical center stays where it is relative to the baseline.
    fn draw_scaled_down(
        &self,
        c: char,
        glyph: &OutlineGlyph,
        bounds: BoundingBox,
        scale: f32,
    ) -> Vec<Vec<u8>> {
        let hinting = Self::hinting_instance(&self.font, self.font_size * scale);
        let scaled = glyph_bounds(glyph, &hinting).unwrap();
        let dx = (self.raster_width as f32 - (scaled.x_max - scaled.x_min)) / 2.0 - scaled.x_min;
        // Round to keep the hinted outline aligned to the pixel grid.
        let center_shift = (scaled.y_min + scaled.y_max - bounds.y_min - bounds.y_max) / 2.0;
        let dy = self.baseline + center_shift.round();
        self.draw_into_cell(c, glyph, &hinting, 1.0, dx, dy)
    }

    /// Turns a glyph outline into the pixels of a cell.
    ///
    /// The outline is compressed horizontally by `scale_x` and its origin is
    /// placed at `(dx, dy)` in the cell. Panics if a visible part of the glyph
    /// is outside the cell.
    fn draw_into_cell(
        &self,
        c: char,
        glyph: &OutlineGlyph,
        hinting: &HintingInstance,
        scale_x: f32,
        dx: f32,
        dy: f32,
    ) -> Vec<Vec<u8>> {
        // The margin catches parts of the glyph outside the cell.
        let margin = self.raster_height;
        let mut rasterizer = Rasterizer::new(
            self.raster_width + 2 * margin,
            self.raster_height + 2 * margin,
        );
        let mut pen = RasterPen {
            rasterizer: &mut rasterizer,
            scale_x,
            dx: dx + margin as f32,
            dy: dy + margin as f32,
            start: point(0.0, 0.0),
            current: point(0.0, 0.0),
        };
        glyph
            .draw(DrawSettings::hinted(hinting, false), &mut pen)
            .unwrap();

        let mut raster = vec![vec![0_u8; self.raster_width]; self.raster_height];
        let mut lost_coverage = 0.0;
        rasterizer.for_each_pixel_2d(|x, y, coverage| {
            let coverage = coverage.clamp(0.0, 1.0);
            let x = (x as usize)
                .checked_sub(margin)
                .filter(|&x| x < self.raster_width);
            let y = (y as usize)
                .checked_sub(margin)
                .filter(|&y| y < self.raster_height);
            match (x, y) {
                (Some(x), Some(y)) => raster[y][x] = (coverage * 255.0).round() as u8,
                _ => lost_coverage += coverage,
            }
        });
        assert!(
            lost_coverage < 0.5,
            "glyph {c:?} exceeds the raster of height {}",
            self.raster_height
        );

        raster
    }

    pub const fn raster_height(&self) -> usize {
        self.raster_height
    }

    pub const fn raster_width(&self) -> usize {
        self.raster_width
    }

    pub const fn font_size(&self) -> f32 {
        self.font_size
    }
}

/// Returns the bounds of a hinted glyph outline in pixels, or `None` if empty.
///
/// The bounds include control points, so they may be slightly too large.
fn glyph_bounds(glyph: &OutlineGlyph, hinting: &HintingInstance) -> Option<BoundingBox> {
    let mut pen = ControlBoundsPen::default();
    glyph
        .draw(DrawSettings::hinted(hinting, false), &mut pen)
        .unwrap();
    pen.bounding_box()
}

/// Pen that draws a glyph outline into the rasterizer.
///
/// It converts from glyph coordinates (origin on the baseline, y up) to the
/// rasterizer's (origin at the top left, y down): `(dx + x * scale_x, dy - y)`.
struct RasterPen<'r> {
    /// Accumulates the coverage of the drawn lines and curves.
    rasterizer: &'r mut Rasterizer,
    /// Horizontal compression factor (1.0: none).
    scale_x: f32,
    /// X coordinate of the glyph origin in the rasterizer.
    dx: f32,
    /// Y coordinate of the glyph origin (on the baseline) in the rasterizer.
    dy: f32,
    /// First point of the current contour, to close it.
    start: Point,
    /// End point of the last drawn segment, where the next one starts.
    current: Point,
}

impl RasterPen<'_> {
    fn point(&self, x: f32, y: f32) -> Point {
        point(x.mul_add(self.scale_x, self.dx), self.dy - y)
    }
}

impl OutlinePen for RasterPen<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.start = self.point(x, y);
        self.current = self.start;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let p = self.point(x, y);
        self.rasterizer.draw_line(self.current, p);
        self.current = p;
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        let (c0, p) = (self.point(cx0, cy0), self.point(x, y));
        self.rasterizer.draw_quad(self.current, c0, p);
        self.current = p;
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let (c0, c1, p) = (self.point(cx0, cy0), self.point(cx1, cy1), self.point(x, y));
        self.rasterizer.draw_cubic(self.current, c0, c1, p);
        self.current = p;
    }

    fn close(&mut self) {
        self.rasterizer.draw_line(self.current, self.start);
        self.current = self.start;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_font_props() {
        let props = RasterizationInfo::new(16, NOTO_SANS_MONO_REGULAR);
        assert_eq!(props.raster_height(), 16);
        assert_eq!(props.raster_width(), 7);
        assert_eq!(props.baseline, 13.0);
    }

    /// Whitespace has no outline and must result in an empty raster.
    #[test]
    fn test_rasterize_whitespace() {
        let props = RasterizationInfo::new(16, NOTO_SANS_MONO_REGULAR);
        let raster = props.rasterize(' ');
        assert_eq!(raster.len(), 16);
        assert!(raster.iter().all(|row| row.len() == 7));
        assert!(raster.iter().flatten().all(|&px| px == 0));
    }

    /// 'H' and 'x' end on the same row, as all glyphs share one baseline.
    #[test]
    fn test_rasterize_common_baseline() {
        let props = RasterizationInfo::new(16, NOTO_SANS_MONO_REGULAR);
        let last_ink_row = |c| {
            let raster = props.rasterize(c);
            raster
                .iter()
                .rposition(|row| row.iter().any(|&px| px > 0))
                .unwrap()
        };
        assert_eq!(last_ink_row('H'), 12);
        assert_eq!(last_ink_row('x'), 12);
        // Thanks to the hinting, the stems of 'H' end sharply.
        let raster = props.rasterize('H');
        assert_eq!(raster[12], raster[11]);
    }

    /// Glyphs much wider than the cell are scaled down rather than cut off.
    #[test]
    fn test_rasterize_wide_glyph() {
        let props = RasterizationInfo::new(16, NOTO_SANS_MONO_REGULAR);
        let raster = props.rasterize('\u{fffd}');
        let left = raster.iter().map(|row| row[0]).max().unwrap();
        let right = raster.iter().map(|row| row[6]).max().unwrap();
        assert!(left > 0 && left < 255);
        assert!(right > 0 && right < 255);
    }

    /// Glyphs slightly wider than the cell are compressed but keep their height.
    #[test]
    fn test_rasterize_slightly_wide_glyph() {
        let props = RasterizationInfo::new(16, NOTO_SANS_MONO_BOLD);
        let ink_rows = |c| {
            let raster = props.rasterize(c);
            let first = raster.iter().position(|row| row.iter().any(|&px| px > 0));
            let last = raster.iter().rposition(|row| row.iter().any(|&px| px > 0));
            (first.unwrap(), last.unwrap())
        };
        // 'ď' (U+010F) is 'd' with a caron on its right side.
        assert_eq!(ink_rows('\u{10f}').1, ink_rows('d').1);
        assert_eq!(ink_rows('\u{10f}').0, ink_rows('d').0);
    }
}
