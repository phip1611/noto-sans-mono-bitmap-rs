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
    clippy::fallible_impl_from
)]
// this comes from the minifb dependency and I can't do anything about it
#![allow(clippy::multiple_crate_versions)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

use codegen::bytes_outsourcer::{BytesToFileOutsourcer, Context};
use codegen::font::{noto_font_by_weight, FontWeight, RasterizationInfo, SUPPORTED_FONT_WEIGHTS};
use codegen::{
    CARGO_LIB_RS, CARGO_TOML_TEMPLATE, CODEGEN_BASE_PATH, CODEGEN_RASTERIZED_BYTES_PATH,
    SIZE_MOD_TEMPLATE, SUPPORTED_RASTER_HEIGHTS, SUPPORTED_UNICODE_RANGES, WEIGHT_MOD_TEMPLATE,
};
use std::fmt::Write as FmtWrite;
use std::fs::{create_dir, File};
use std::io::Write as IoWrite;
use std::path::PathBuf;

/// Binary that does all the codegen.
fn main() {
    // Shared instance for the font rasterization of all characters. Ensures unique filenames
    // so that I can `include!` the Rust definitions for the rasterized characters.
    let mut bytes_outsourcer = BytesToFileOutsourcer::new(CODEGEN_RASTERIZED_BYTES_PATH);

    // debugging info
    {
        let font =
            RasterizationInfo::new(20 as usize, noto_font_by_weight(&SUPPORTED_FONT_WEIGHTS[0]));
        println!("INFO: The widest char is '{}'", font.widest_char());
    }

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

/// Creates a `<weight>/size_<size>.rs` file performs all the code generation for the byte look-up of
/// the pre-rasterized characters.
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
    {
        writeln!(
            &mut size_mod_file,
            "{}",
            SIZE_MOD_TEMPLATE
                .replace("%FONT_WEIGHT%", weight.mod_name())
                .replace("%FONT_SIZE%", &format!("{}", font.raster_height()))
                .replace(
                    "%CODEGEN_RASTER_HEIGHT%",
                    &format!("{}", font.raster_height())
                )
                .replace(
                    "%CODEGEN_RASTER_WIDTH%",
                    &format!("{}", font.raster_width())
                )
        )
        .unwrap();
    }

    // the rest of the file generates the big match-block that maps characters to the pre-rasterized
    // bytes.

    // prepares the "get_char" function with its match block
    let mut code_range_string = String::new();
    {
        writeln!(
            &mut code_range_string,
            "/// Returns the raster of the given character for font weight {} and font size {}px.\n\
            /// Wide characters, such as '�', will be truncated in their width in order to fullfill\n\
            /// the mono font guarantee. All characters are centered in their raster.",
            weight.mod_name(),
            font.font_size().round()
        )
        .unwrap();
        writeln!(&mut code_range_string, "#[inline]").unwrap();
        writeln!(
            &mut code_range_string,
            "pub const fn get_char(c: char) -> Option<&'static [&'static [u8]]> {{"
        )
        .unwrap();
        writeln!(&mut code_range_string, "    match c {{").unwrap();
    }

    // now we generate all the single match arms per character

    // iterates through all ranges and for each range over all visible characters
    SUPPORTED_UNICODE_RANGES.iter().for_each(|range| {
        range
            .iter()
            .map(|char| (char, font.rasterize(char)))
            .for_each(|(char, raster)| {
                writeln!(
                    &mut code_range_string,
                    "        // letter: '{}' / {:?}",
                    char, char as usize as *const usize
                )
                .unwrap();

                // make this character optional by it's unicode range
                writeln!(
                    &mut code_range_string,
                    "#[cfg(feature = \"{}\")]",
                    range.feature_name
                )
                .unwrap();

                // generate source code representation of raster
                let rust_raster_source_code = codegen_raster(&raster);

                let outsourced_path = outsourcer.outsource_bytes(
                    rust_raster_source_code.as_bytes(),
                    Context {
                        c: char,
                        weight: weight.clone(),
                        height: font.raster_height() as u32,
                    },
                );

                // generate left side of the match arm
                {
                    if char == '\\' || char == '\'' {
                        write!(&mut code_range_string, "        '\\{}'", char,).unwrap();
                    } else {
                        write!(&mut code_range_string, "        '{}'", char,).unwrap()
                    }
                }
                // generate right side of the match arm
                {
                    // this is (as the rest of the codegen stuff) very ugly.
                    // need to adapt the path, so that cargo can find it during compilation..
                    let path = format!(
                        "../res_rasterized_characters/{}",
                        outsourced_path.file_name().unwrap().to_str().unwrap()
                    );
                    writeln!(&mut code_range_string, "=> Some(include!(\"{}\")),", path).unwrap();
                }
            })
    });
    writeln!(&mut code_range_string, "        _ => None").unwrap();
    // close match
    writeln!(&mut code_range_string, "    }}").unwrap();
    // close function
    writeln!(&mut code_range_string, "}}").unwrap();

    size_mod_file
        .write_all(code_range_string.as_bytes())
        .unwrap();
}

/// Generates the Rust source code of type `&[&u8]]` from a raster.
fn codegen_raster(raster: &Vec<Vec<u8>>) -> String {
    let mut rust_byte_array_str = String::new();
    writeln!(&mut rust_byte_array_str, "&[").unwrap();
    for row in raster {
        write!(&mut rust_byte_array_str, "    &[").unwrap();
        for (i, byte) in row.iter().enumerate() {
            if i == row.len() - 1 {
                write!(&mut rust_byte_array_str, "{}", byte).unwrap();
            } else {
                write!(&mut rust_byte_array_str, "{}, ", byte).unwrap();
            }
        }
        writeln!(&mut rust_byte_array_str, "],").unwrap();
    }
    write!(&mut rust_byte_array_str, "]").unwrap();
    rust_byte_array_str
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_codegen_raster() {
        let raster = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        let generated_source_code = codegen_raster(&raster);
        assert_eq!(
            "&[\n\
            \x20\x20\x20\x20&[1, 2, 3],\n\
            \x20\x20\x20\x20&[4, 5, 6],\n\
            \x20\x20\x20\x20&[7, 8, 9],\n\
            ]",
            &generated_source_code
        );
    }
}
