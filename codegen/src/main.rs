#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// this comes from the minifb dependency and I can't do anything about it
#![allow(clippy::multiple_crate_versions)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]

use codegen::bytes_outsourcer::{BytesToFileOutsourcer, Context};
use codegen::font::{FontWeight, RasterizationInfo, SUPPORTED_FONT_WEIGHTS, noto_font_by_weight};
use codegen::unicode::UnicodeRange;
use codegen::{
    CARGO_LIB_RS, CARGO_TOML_TEMPLATE, CODEGEN_BASE_PATH, CODEGEN_RASTERIZED_BYTES_DIR_NAME,
    CODEGEN_RASTERIZED_BYTES_PATH, SIZE_MOD_TEMPLATE, SUPPORTED_RASTER_HEIGHTS,
    SUPPORTED_UNICODE_RANGES, WEIGHT_MOD_TEMPLATE,
};
use std::fmt::Write as FmtWrite;
use std::fs::{File, create_dir, create_dir_all};
use std::io::Write as IoWrite;
use std::ops::RangeInclusive;
use std::path::PathBuf;

/// Binary that does all the codegen.
fn main() {
    // Shared instance for the font rasterization of all characters. Ensures unique filenames
    // so that I can `include_bytes!` the binary blobs with the rasterized characters.
    create_dir_all(CODEGEN_RASTERIZED_BYTES_PATH).unwrap();
    let mut bytes_outsourcer = BytesToFileOutsourcer::new(CODEGEN_RASTERIZED_BYTES_PATH);

    // create the font weight modules for each supported font weight.
    for weight in SUPPORTED_FONT_WEIGHTS {
        let font_bytes = noto_font_by_weight(weight);
        codegen_font_weight_module(font_bytes, weight, &mut bytes_outsourcer);
    }

    codegen_cargo_toml();
    codegen_lib_rs();
}

/// Generates the Cargo.toml with all relevant features.
fn codegen_cargo_toml() {
    // let mut path = PathBuf::from(CODEGEN_BASE_PATH);
    let mut path = PathBuf::from("..");
    path.push("Cargo.toml");

    let mut cargo_toml_file = File::options()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(path)
        .unwrap();

    let mut features_font_weights = String::new();
    SUPPORTED_FONT_WEIGHTS.iter().for_each(|w| {
        writeln!(&mut features_font_weights, "{} = []", w.mod_name()).unwrap();
    });

    let mut features_font_sizes = String::new();
    SUPPORTED_RASTER_HEIGHTS.iter().for_each(|size| {
        writeln!(&mut features_font_sizes, "{} = []", size.feature_name()).unwrap();
    });

    let mut features_unicode_ranges = String::new();
    SUPPORTED_UNICODE_RANGES
        .iter()
        .map(|r| r.feature_name)
        .for_each(|name| {
            writeln!(&mut features_unicode_ranges, "{} = []", name).unwrap();
        });

    let mut features_font_styles_default = String::new();
    writeln!(
        &mut features_font_styles_default,
        "font_weights_default = ["
    )
    .unwrap();
    SUPPORTED_FONT_WEIGHTS
        .iter()
        .filter(|x| x.default_feature())
        .for_each(|w| {
            writeln!(
                &mut features_font_styles_default,
                "    \"{}\",",
                w.mod_name()
            )
            .unwrap();
        });
    writeln!(&mut features_font_styles_default, "]").unwrap();

    let mut features_font_styles_all = String::new();
    writeln!(&mut features_font_styles_all, "font_weights_all = [").unwrap();
    SUPPORTED_FONT_WEIGHTS.iter().for_each(|w| {
        writeln!(&mut features_font_styles_all, "    \"{}\",", w.mod_name()).unwrap();
    });
    writeln!(&mut features_font_styles_all, "]").unwrap();

    let mut features_unicode_default = String::new();
    writeln!(&mut features_unicode_default, "unicode_ranges_default = [").unwrap();
    SUPPORTED_UNICODE_RANGES
        .iter()
        .filter(|r| r.default_feature)
        .map(|r| r.feature_name)
        .for_each(|name| {
            writeln!(&mut features_unicode_default, "    \"{}\",", name).unwrap();
        });
    writeln!(&mut features_unicode_default, "]").unwrap();

    let mut features_unicode_all = String::new();
    writeln!(&mut features_unicode_all, "unicode_ranges_all = [").unwrap();
    SUPPORTED_UNICODE_RANGES
        .iter()
        .map(|r| r.feature_name)
        .for_each(|name| {
            writeln!(&mut features_unicode_all, "    \"{}\",", name).unwrap();
        });
    writeln!(&mut features_unicode_all, "]").unwrap();

    let mut features_raster_heights_default = String::new();
    writeln!(
        &mut features_raster_heights_default,
        "raster_heights_default = ["
    )
    .unwrap();
    SUPPORTED_RASTER_HEIGHTS
        .iter()
        .filter(|r| r.default_feature())
        .for_each(|size| {
            writeln!(
                &mut features_raster_heights_default,
                "    \"{}\",",
                size.feature_name()
            )
            .unwrap();
        });
    writeln!(&mut features_raster_heights_default, "]").unwrap();

    let mut features_raster_heights_all = String::new();
    writeln!(&mut features_raster_heights_all, "raster_heights_all = [").unwrap();
    SUPPORTED_RASTER_HEIGHTS.iter().for_each(|size| {
        writeln!(
            &mut features_raster_heights_all,
            "    \"{}\",",
            size.feature_name()
        )
        .unwrap();
    });
    writeln!(&mut features_raster_heights_all, "]").unwrap();

    // replace placeholders
    #[rustfmt::skip]
    writeln!(
        &mut cargo_toml_file,
        "{}",
        CARGO_TOML_TEMPLATE
            .replace("# %CODEGEN_FONT_WEIGHTS%", features_font_weights.as_str())
            .replace("# %CODEGEN_FONT_SIZES%", features_font_sizes.as_str())
            .replace("# %CODEGEN_UNICODE_RANGES%", features_unicode_ranges.as_str())
            .replace("# %CODEGEN_FEATURES_RASTER_HEIGHTS_DEFAULT%", features_raster_heights_default.as_str())
            .replace("# %CODEGEN_FEATURES_RASTER_HEIGHTS_ALL%", features_raster_heights_all.as_str())
            .replace("# %CODEGEN_FEATURES_WEIGHTS_DEFAULT%", features_font_styles_default.as_str())
            .replace("# %CODEGEN_FEATURES_WEIGHTS_ALL%", features_font_styles_all.as_str())
            .replace("# %CODEGEN_FEATURES_UNICODE_RANGES_DEFAULT%", features_unicode_default.as_str())
            .replace("# %CODEGEN_FEATURES_UNICODE_RANGES_ALL%", features_unicode_all.as_str())
    )
    .unwrap();
}

/// Generates the lib.rs with all relevant features.
fn codegen_lib_rs() {
    let mut lib_rs_path = PathBuf::from(CODEGEN_BASE_PATH);
    lib_rs_path.push("lib.rs");

    let mut cargo_toml_file = File::options()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(lib_rs_path)
        .unwrap();

    // codegen font weight modules
    let mut weight_modules = String::new();
    {
        SUPPORTED_FONT_WEIGHTS.iter().for_each(|w| {
            writeln!(
                &mut weight_modules,
                "#[cfg(feature = \"{}\")]",
                w.mod_name()
            )
            .unwrap();
            writeln!(&mut weight_modules, "mod {};", w.mod_name()).unwrap();
        });
    }

    // codegen font weight variants
    let mut weight_variants = String::new();
    {
        SUPPORTED_FONT_WEIGHTS.iter().for_each(|w| {
            writeln!(
                &mut weight_variants,
                "    #[cfg(feature = \"{}\")]",
                w.mod_name()
            )
            .unwrap();
            writeln!(&mut weight_variants, "    {:?},", w.name()).unwrap();
        });
    }

    // codegen font size enum variants
    let mut font_size_enum_variants = String::new();
    {
        SUPPORTED_RASTER_HEIGHTS.iter().for_each(|height| {
            writeln!(
                &mut font_size_enum_variants,
                "    #[cfg(feature = \"{}\")]",
                height.feature_name()
            )
            .unwrap();
            writeln!(
                &mut font_size_enum_variants,
                "    Size{} = {},",
                height.value(),
                height.value()
            )
            .unwrap();
        });
    }

    // codegen get_raster match
    let mut get_raster_match = String::new();
    {
        SUPPORTED_FONT_WEIGHTS.iter().for_each(|w| {
            writeln!(
                &mut get_raster_match,
                "        #[cfg(feature = \"{}\")]",
                w.mod_name()
            )
            .unwrap();
            writeln!(
                &mut get_raster_match,
                "        FontWeight::{:?} => match size {{",
                w.name()
            )
            .unwrap();
            SUPPORTED_RASTER_HEIGHTS.iter().for_each(|size| {
                writeln!(
                    &mut get_raster_match,
                    "            #[cfg(feature = \"{}\")]",
                    size.feature_name()
                )
                .unwrap();
                writeln!(
                    &mut get_raster_match,
                    "            RasterHeight::Size{:?} => crate::{}::size_{}::get_char(c),",
                    size.value(),
                    w.mod_name(),
                    size.value()
                )
                .unwrap();
            });
            writeln!(&mut get_raster_match, "        }}").unwrap();
        });
    }

    // codegen get_raster_width match
    let mut get_raster_width_match = String::new();
    {
        SUPPORTED_FONT_WEIGHTS.iter().for_each(|w| {
            writeln!(
                &mut get_raster_width_match,
                "        #[cfg(feature = \"{}\")]",
                w.mod_name()
            )
            .unwrap();
            writeln!(
                &mut get_raster_width_match,
                "        FontWeight::{:?} => match size {{",
                w.name()
            )
            .unwrap();
            SUPPORTED_RASTER_HEIGHTS.iter().for_each(|size| {
                writeln!(
                    &mut get_raster_width_match,
                    "            #[cfg(feature = \"{}\")]",
                    size.feature_name()
                )
                .unwrap();
                writeln!(
                    &mut get_raster_width_match,
                    "            RasterHeight::Size{} => crate::{}::size_{}::RASTER_WIDTH,",
                    size.value(),
                    w.mod_name(),
                    size.value()
                )
                .unwrap();
            });
            writeln!(&mut get_raster_width_match, "        }}").unwrap();
        });
    }

    // codegen the lists of enabled font weights, raster heights, and supported
    // chars for the unit tests of the generated crate
    let mut test_font_weights = String::new();
    SUPPORTED_FONT_WEIGHTS.iter().for_each(|w| {
        writeln!(
            &mut test_font_weights,
            "        #[cfg(feature = \"{}\")]",
            w.mod_name()
        )
        .unwrap();
        writeln!(
            &mut test_font_weights,
            "        FontWeight::{:?},",
            w.name()
        )
        .unwrap();
    });
    let mut test_raster_heights = String::new();
    SUPPORTED_RASTER_HEIGHTS.iter().for_each(|size| {
        writeln!(
            &mut test_raster_heights,
            "        #[cfg(feature = \"{}\")]",
            size.feature_name()
        )
        .unwrap();
        writeln!(
            &mut test_raster_heights,
            "        RasterHeight::Size{},",
            size.value()
        )
        .unwrap();
    });
    let mut test_char_segments = String::new();
    SUPPORTED_UNICODE_RANGES.iter().for_each(|range| {
        for segment in range.segments() {
            writeln!(
                &mut test_char_segments,
                "        #[cfg(feature = \"{}\")]",
                range.feature_name
            )
            .unwrap();
            writeln!(
                &mut test_char_segments,
                "        RangeInclusive::new('\\u{{{:x}}}', '\\u{{{:x}}}'),",
                *segment.chars.start() as u32,
                *segment.chars.end() as u32
            )
            .unwrap();
        }
    });

    // replace placeholders
    {
        writeln!(
            &mut cargo_toml_file,
            "{}",
            CARGO_LIB_RS
                .replace("// %CODEGEN_LIB_MODULES%", weight_modules.as_str())
                .replace(
                    "// %CODEGEN_FONT_WEIGHT_VARIANTS%",
                    weight_variants.as_str()
                )
                .replace(
                    "// %CODEGEN_RASTER_SIZE_ENUM_VARIANTS%",
                    font_size_enum_variants.as_str()
                )
                .replace("// %CODEGEN_get_raster%", get_raster_match.as_str())
                .replace(
                    "// %CODEGEN_get_raster_WIDTH%",
                    get_raster_width_match.as_str()
                )
                .replace("// %CODEGEN_TEST_FONT_WEIGHTS%", test_font_weights.as_str())
                .replace(
                    "// %CODEGEN_TEST_RASTER_HEIGHTS%",
                    test_raster_heights.as_str()
                )
                .replace(
                    "// %CODEGEN_TEST_CHAR_SEGMENTS%",
                    test_char_segments.as_str()
                )
        )
        .unwrap();
    }
}

/// Creates a font weight module, like `bold/mod.rs`.
fn codegen_font_weight_module(
    font_bytes: &[u8],
    weight: &FontWeight,
    outsourcer: &mut BytesToFileOutsourcer,
) {
    let mut mod_file_path = PathBuf::from(CODEGEN_BASE_PATH);
    mod_file_path.push(weight.mod_name());
    // ignore error; dir might exist
    let _ = create_dir(&mod_file_path);
    mod_file_path.push("mod.rs");

    let mut mod_file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .append(false)
        .open(mod_file_path)
        .unwrap();

    // replace placeholders
    writeln!(
        &mut mod_file,
        "{}",
        WEIGHT_MOD_TEMPLATE.replace("%FONT_WEIGHT%", weight.mod_name())
    )
    .unwrap();

    for size in SUPPORTED_RASTER_HEIGHTS.iter().map(|x| x.value()) {
        // add font modules for the font sizes
        writeln!(&mut mod_file, "#[cfg(feature = \"size_{}\")]", size).unwrap();
        writeln!(&mut mod_file, "pub mod size_{};", size).unwrap();

        let font = RasterizationInfo::new(size as usize, font_bytes);
        codegen_font_weight_sub_modules(font, weight, outsourcer);
    }
}

/// Creates a `<weight>/size_<size>.rs` file and the binary glyph blobs (one per unicode range) it
/// includes, and performs all the code generation for the look-up of the pre-rasterized
/// characters.
fn codegen_font_weight_sub_modules(
    font: RasterizationInfo,
    weight: &FontWeight,
    outsourcer: &mut BytesToFileOutsourcer,
) {
    // this block creates the file <weight>/size_<size>.rs and returns a File object to that file.
    let mut size_mod_file = {
        let mut mod_file_path = PathBuf::from(CODEGEN_BASE_PATH);
        mod_file_path.push(weight.mod_name());
        mod_file_path.push(format!("size_{}.rs", font.raster_height()));

        File::options()
            .create(true)
            .write(true)
            .append(false)
            .truncate(true)
            .open(mod_file_path)
            .unwrap()
    };

    // this block prepares the head of the just generated file
    let mut code = SIZE_MOD_TEMPLATE
        .replace("%FONT_WEIGHT%", weight.mod_name())
        .replace("%FONT_SIZE%", &format!("{}", font.raster_height()))
        .replace(
            "%CODEGEN_RASTER_HEIGHT%",
            &format!("{}", font.raster_height()),
        )
        .replace(
            "%CODEGEN_RASTER_WIDTH%",
            &format!("{}", font.raster_width()),
        );

    // The match arms of `get_char`: one per segment of consecutive code points.
    let mut match_arms = String::new();

    // For each unicode range: one blob with all glyphs back to back, one static that includes
    // the blob, and the match arms that map chars to the index of their glyph in the blob.
    for range in SUPPORTED_UNICODE_RANGES {
        let blob = codegen_glyph_blob(&font, range);
        let glyph_count = range.iter().count();

        let outsourced_path = outsourcer.outsource_bytes(
            &blob,
            Context {
                weight: *weight,
                height: font.raster_height() as u32,
                unicode_range: range.feature_name,
            },
        );
        let file_name = outsourced_path.file_name().unwrap().to_str().unwrap();

        let static_name = range.feature_name.to_uppercase().replace('-', "_");

        // `1 * GLYPH_LEN` would trigger `clippy::identity_op`
        let blob_len = if glyph_count == 1 {
            "GLYPH_LEN".to_string()
        } else {
            format!("{glyph_count} * GLYPH_LEN")
        };

        writeln!(&mut code).unwrap();
        writeln!(
            &mut code,
            "/// The glyphs of the unicode range `{}` ({glyph_count} in total), back to back.",
            range.feature_name
        )
        .unwrap();
        writeln!(&mut code, "#[cfg(feature = \"{}\")]", range.feature_name).unwrap();
        writeln!(
            &mut code,
            "static {static_name}: [u8; {blob_len}] = \
            *include_bytes!(\"../{}/{file_name}\");",
            CODEGEN_RASTERIZED_BYTES_DIR_NAME
        )
        .unwrap();

        for segment in range.segments() {
            let (first_index, first_char) = (segment.first_index, *segment.chars.start() as u32);
            // Index of the glyph in the blob.
            let index_expr = if segment.chars.start() == segment.chars.end() {
                format!("{first_index}")
            } else if first_index == 0 {
                format!("c as usize - 0x{first_char:x}")
            } else {
                format!("c as usize - 0x{first_char:x} + {first_index}")
            };
            writeln!(
                &mut match_arms,
                "        #[cfg(feature = \"{}\")]",
                range.feature_name
            )
            .unwrap();
            writeln!(
                &mut match_arms,
                "        {} => Some(crate::glyph_at(&{static_name}, GLYPH_LEN, {index_expr})),",
                char_range_pattern(&segment.chars)
            )
            .unwrap();
        }
    }

    // the "get_char" function with its match block
    writeln!(&mut code).unwrap();
    writeln!(
        &mut code,
        "/// Returns the raster of the given character for font weight {} and font size {}px.\n\
        /// Wide characters, such as '�', are scaled down to fit into the raster in order to\n\
        /// fulfill the mono font guarantee. All characters share a common baseline.\n\
        ///\n\
        /// The returned slice is exactly [`GLYPH_LEN`] bytes in row-major order.",
        weight.mod_name(),
        font.font_size().round()
    )
    .unwrap();
    writeln!(&mut code, "#[inline]").unwrap();
    writeln!(
        &mut code,
        "pub const fn get_char(c: char) -> Option<&'static [u8]> {{"
    )
    .unwrap();
    writeln!(&mut code, "    match c {{").unwrap();
    code.push_str(&match_arms);
    writeln!(&mut code, "        _ => None,").unwrap();
    writeln!(&mut code, "    }}").unwrap();
    writeln!(&mut code, "}}").unwrap();

    size_mod_file.write_all(code.as_bytes()).unwrap();
}

/// Rasterizes all chars of the unicode range and returns their glyphs back to back, in the
/// order of [`UnicodeRange::iter`].
fn codegen_glyph_blob(font: &RasterizationInfo, range: &UnicodeRange) -> Vec<u8> {
    let (width, height) = (font.raster_width(), font.raster_height());
    let mut blob = Vec::with_capacity(range.iter().count() * width * height);
    for c in range.iter() {
        append_glyph(&mut blob, &font.rasterize(c), width, height);
    }
    blob
}

/// Appends the raster (a vector of rows) of a glyph to the blob in flat row-major order. Asserts
/// that the raster has exactly the given dimensions.
fn append_glyph(blob: &mut Vec<u8>, raster: &[Vec<u8>], width: usize, height: usize) {
    assert_eq!(
        raster.len(),
        height,
        "raster must have exactly {height} rows"
    );
    for row in raster {
        assert_eq!(
            row.len(),
            width,
            "each row must have exactly {width} pixels"
        );
        blob.extend_from_slice(row);
    }
}

/// Returns a Rust pattern that matches the chars, such as `'\u{a1}'..='\u{ac}'`.
fn char_range_pattern(chars: &RangeInclusive<char>) -> String {
    if chars.start() == chars.end() {
        format!("'\\u{{{:x}}}'", *chars.start() as u32)
    } else {
        format!(
            "'\\u{{{:x}}}'..='\\u{{{:x}}}'",
            *chars.start() as u32,
            *chars.end() as u32
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_append_glyph() {
        let mut blob = vec![42];
        let raster = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec()];
        append_glyph(&mut blob, &raster, 3, 2);
        assert_eq!(blob, [42, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    #[should_panic]
    fn test_append_glyph_wrong_width() {
        let raster = [[1, 2, 3].to_vec(), [4, 5].to_vec()];
        append_glyph(&mut Vec::new(), &raster, 3, 2);
    }

    #[test]
    fn test_char_range_pattern() {
        assert_eq!(
            char_range_pattern(&('\u{a1}'..='\u{ac}')),
            "'\\u{a1}'..='\\u{ac}'"
        );
        assert_eq!(char_range_pattern(&('�'..='�')), "'\\u{fffd}'");
    }
}
