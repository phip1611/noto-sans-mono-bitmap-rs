use fontdue::{Font, FontSettings};

const NOTO_SANS_MONO_REGULAR: &[u8] = include_bytes!("res/NotoSansMono-Regular.ttf");
const NOTO_SANS_MONO_BOLD: &[u8] = include_bytes!("res/NotoSansMono-Bold.ttf");
const NOTO_SANS_MONO_LIGHT: &[u8] = include_bytes!("res/NotoSansMono-Light.ttf");

/// Additional horizontal padding for each rasterized character in its raster/box. All chars
/// will be centered in that box. With this value, we can influence the spacing of the text
/// if multiple characters are displayed side by side.
const RASTERIZED_FONT_ADDITIONAL_PADDING: usize = 0;

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
/// Guarantees, that each font raster centers the letter in a vertical and horizontal way.
/// It might truncate letters to the left or right, especially for wide characters, such as '�'.
/// There will be a small vertical padding to other lines (if rendered as multiline) but almost no
/// padding to the left and right by default. This way, letters can be displayed side by side and
/// appear as mono-space font.
///
/// The raster is not XxX but XxY, because a mono font not necessarily needs to be XxX, as long as
/// each character has the same width.
#[derive(Debug)]
pub struct RasterizationInfo {
    /// See [`Font`].
    font: Font,
    /// Height of the raster for the font rasterization.
    raster_height: usize,
    /// Width of each letter for the font rasterization.
    /// This width ensures that the font is indeed a mono-space font.
    raster_width: usize,
    /// This is a little lower value than the height and is required to place the
    /// letter inside the box without clipping at the top or at the bottom.
    font_size: f32,
    /// The widest char/symbol after rasterization. Interesting as debug information which
    /// letter is the widest. Helps me to streamling the width for the centering of all letters.
    widest_char: char,
}

impl RasterizationInfo {
    /// Creates a new object, ready to rasterize characters into a raster.
    ///
    /// # Parameters
    /// * `raster_height` height of the raster. A little bit bigger than the font on the screen.
    ///                   Values are for example 14, 16, 24,.
    /// * `font_bytes` Raw bytes of a font file that [`fontdue`] can parse.
    pub fn new(raster_height: usize, font_bytes: &[u8]) -> Self {
        let line_height = raster_height as f32;

        // Work out the ratio between the raster_height (e.g. 16) and the line height
        // with ascenders and descenders (e.g. 23).
        let font_size_to_raster_height_ratio = line_height / {
            // Get font metrics.
            let font_face = ttf_parser::Face::parse(font_bytes, 0).unwrap();
            let ascender = font_face.ascender() as i32;
            let descender = font_face.descender() as i32;
            let line_gap = font_face.line_gap() as i32;
            let units_per_em = font_face.units_per_em() as f32;
            let scale = line_height / units_per_em;
            // Get line height plus ascenders and descenders.
            (ascender - descender + line_gap) as f32 * scale
        };

        // Work out the font size that fits text inside the raster_height.
        let font_size = (line_height * font_size_to_raster_height_ratio).floor();

        let font = Font::from_bytes(
            font_bytes,
            FontSettings {
                scale: font_size,
                ..Default::default()
            },
        )
        .unwrap();

        let (widest_char, raster_width) = Self::find_max_width(&font, font_size);

        Self {
            font,
            raster_height,
            raster_width,
            font_size,
            widest_char,
        }
    }

    /// Rasterizes a char for the given [`Self`] object into a raster/box. Every letter in the
    /// resulting mono font is horizontal and vertical aligned to the center. Furthermore,
    /// the resulting mono font contains already a vertical line spacing of a few pixels, but
    /// almost no padding to the left and right. This way, letters can be displayed side by side
    /// and appear as mono-space font.
    pub fn rasterize(&self, c: char) -> Vec<Vec<u8>> {
        let font_size = self.font_size;
        let (metrics, fontdue_bitmap) = self.font.rasterize(c, font_size);

        // the bitmap that will contain the properly aligned rasterized char
        let mut letter_bitmap = vec![vec![0_u8; self.raster_width]; self.raster_height];

        // align to horizontal center
        let x_offset = (metrics.xmin as f32
            + (self.raster_width as f32 - metrics.advance_width as f32) / 2.0)
            .floor() as isize;

        // align to vertical center
        let y_offset = ((font_size - metrics.height as f32) - metrics.ymin as f32).round() as isize;

        for ((y, x), intensity, skip) in fontdue_bitmap
            .iter()
            .enumerate()
            .map(|(i, p)| (i as isize, p))
            .map(|(i, p)| {
                let x = x_offset + (i % metrics.width as isize);
                let y = y_offset + (i / metrics.width as isize);

                let skip = x >= self.raster_width as isize || y >= self.raster_height as isize;

                // if some letter is "too" big and out of bounds the box: cut and prevent error
                let x = trim_index_to_bounds!(x, self.raster_width());
                let y = trim_index_to_bounds!(y, self.raster_height());

                ((y, x), *p, skip)
            })
        {
            if !skip {
            letter_bitmap[y][x] = intensity;
            }
        }

        letter_bitmap
    }

    /// Searches the maximum width, that a pre-rasterized character/ will have for the given font
    /// size. This way, the width of the final raster can be reduced to HEIGHT x WIDTH instead of
    /// HEIGHT x HEIGHT, which would result a big space between all letters. The pre-rasterized
    /// characters will be all "monospaced" out-of-the box with this approach.
    ///
    /// This function only takes ASCII letters (BASIC_LATIN) into account. Wider symbols are
    /// ignored and will later be truncated/cut to this width.
    fn find_max_width(font: &Font, font_size: f32) -> (char, usize) {
        let (char, max) = crate::unicode::BASIC_LATIN
            .iter()
            // return tuple of char and rasterized width
            .map(|c| (c, font.rasterize(c, font_size).0.width))
            // look for maximum by width
            .max_by(|(_, l_width), (_, r_width)| l_width.cmp(r_width))
            .unwrap();

        (char, max + RASTERIZED_FONT_ADDITIONAL_PADDING)
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

    pub const fn widest_char(&self) -> char {
        self.widest_char
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_font_props() {
        let props = RasterizationInfo::new(16, NOTO_SANS_MONO_REGULAR);
        println!("raster_height = {}", props.raster_height());
        println!("font_size     = {}", props.font_size());
        println!("raster_width  = {}", props.raster_width());
        println!("widest_char   = '{}'", props.widest_char());
    }
}
