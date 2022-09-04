use crate::SUPPORTED_UNICODE_RANGES;
use fontdue::{Font, FontSettings};

const NOTO_SANS_MONO_REGULAR: &[u8] = include_bytes!("res/NotoSansMono-Regular.ttf");
const NOTO_SANS_MONO_BOLD: &[u8] = include_bytes!("res/NotoSansMono-Bold.ttf");
const NOTO_SANS_MONO_LIGHT: &[u8] = include_bytes!("res/NotoSansMono-Light.ttf");

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

    pub fn name(&self) -> &FontWeightName {
        &self.name
    }

    pub fn mod_name(&self) -> &str {
        self.name.mod_name()
    }

    pub fn default_feature(&self) -> bool {
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

/// Makes sure the index is in bounds [0..upper_bound] and
/// of type usize in the end.
macro_rules! trim_index_to_bounds {
    ($num:ident, $bounds:expr) => {
        if ($num as usize) >= $bounds {
            // usually, here it happens that -1 gets truncated to 0 and
            // bounds + 1 back to bounds

            // eprintln!("{} is out of bound {}!", $num, $bounds);
            ($bounds - 1) as usize
        } else if $num < 0 {
            0_usize
        } else {
            $num as usize
        }
    };
}

/// All font-related information to render characters with [`fontdue`]
/// into a bitmap font. Currently, the usage of Noto Sans Mono is hard-coded.
///
/// Guarantees, that each bitmap font raster centers the letter in a vertical
/// and horizontal way. There will be a small vertical padding to other lines
/// (if rendered as multiline) but almost no padding to the left and right by
/// default.
///
/// The raster is not XxX but XxY, because a mono font not necessarily needs to
/// be XxX, as long as each character has the same width.
#[derive(Debug)]
pub struct ToBitmapFont {
    font: Font,
    bitmap_height: usize,
    bitmap_width: usize,
    font_size: f32,
}

impl ToBitmapFont {
    /// Creates a new object, ready to rasterize characters into a bitmap.
    ///
    /// # Parameters
    /// * `bitmap_height` height of the bitmap. A little bit bigger than the font on the screen.
    ///                   Values are for example 14, 16, 24,.
    /// * `font_bytes` Raw bytes of a font file, that [`fontdue`] can parse.
    pub fn new(bitmap_height: usize, font_bytes: &[u8]) -> Self {
        // We need some padding at the top and the bottom of each box, because
        // of letters such as "Ä" and "y". I figured the value out just by trying
        // with my "rasterize_chars_in_window" binary. It depends on the y_offset
        // in `rasterize_to_bitmap()`
        let font_size = (bitmap_height as f32 * 0.84).ceil();

        let font = Font::from_bytes(
            font_bytes,
            FontSettings {
                scale: font_size,
                ..Default::default()
            },
        )
        .unwrap();

        let bitmap_width = Self::find_max_width(&font, font_size);

        Self {
            font,
            bitmap_height,
            bitmap_width,
            font_size,
        }
    }

    /// Rasterizes a char for the given [`ToBitmapFont`] object into a bitmap. Every letter in the
    /// resulting bitmap mono font is horizontal and vertical aligned to the center. Furthermore,
    /// the resulting mono font contains already a vertical line spacing of a few pixels, but no
    /// padding to the left and right.
    pub fn rasterize_to_bitmap(&self, c: char) -> Vec<Vec<u8>> {
        let (metrics, fontdue_bitmap) = self.font.rasterize(c, self.font_size);

        // the bitmap that will contain the properly aligned rasterized char
        let mut letter_bitmap = vec![vec![0_u8; self.bitmap_width]; self.bitmap_height];

        for ((y, x), intensity) in fontdue_bitmap
            .iter()
            .enumerate()
            .map(|(i, p)| (i as isize, p))
            .map(|(i, p)| {
                // align to horizontal center
                let x_offset = (self.bitmap_width as isize - metrics.width as isize) / 2;

                // align to vertical center
                // 1) bounds:height: align big letters to groundline regarding the font size
                let mut y_offset = self.font_size as isize - metrics.height as isize;
                // 2) move downwards, because there are parts "below the ground line"  (like in y)
                y_offset -= metrics.ymin as isize;
                // 3) move everything slightly to the top; I figured this out by trying with
                //    my "rasterize_chars_in_window" binary
                y_offset -= (self.bitmap_height as f32 * 0.07) as isize;

                let x = i % metrics.width as isize;
                let y = i as isize / metrics.width as isize;

                let x = x + x_offset;
                let y = y + y_offset;

                // if some letter is "too" big and out of bounds the box: cut and prevent error
                let x = trim_index_to_bounds!(x, self.bitmap_width());
                let y = trim_index_to_bounds!(y, self.bitmap_height());

                ((y, x), p)
            })
        {
            letter_bitmap[y][x] = *intensity;
        }

        letter_bitmap
    }

    /// A brute force approach to find the maximum width, that a supported unicode
    /// char will have for the given font size. This way, the width of the final
    /// bitmap can be reduced to HEIGHT x WIDTH instead of HEIGHT x HEIGHT, which
    /// would indicate a big space between all letters.
    fn find_max_width(font: &Font, font_size: f32) -> usize {
        SUPPORTED_UNICODE_RANGES
            .iter()
            .flat_map(|range| range.iter())
            .filter(|symbol| symbol.is_visible_char())
            .map(|s| s.get_char())
            .map(|c| (c, font.rasterize(c, font_size).0.width))
            .max()
            .unwrap()
    }

    pub const fn bitmap_height(&self) -> usize {
        self.bitmap_height
    }
    pub const fn bitmap_width(&self) -> usize {
        self.bitmap_width
    }
    pub const fn font_size(&self) -> f32 {
        self.font_size
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_font_props() {
        let props = ToBitmapFont::new(16, NOTO_SANS_MONO_REGULAR);
        println!("bitmap_height = {}", props.bitmap_height());
        println!("bitmap_width  = {}", props.bitmap_width());
        println!("font_size     = {}", props.font_size());
    }
}
