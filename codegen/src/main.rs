use codegen::font::{noto_font_by_weight, FontWeight, ToBitmapFont};
use codegen::unicode::UnicodeIter;
use codegen::{
    BitmapHeight, CARGO_LIB_RS, CARGO_TOML_TEMPLATE, CODEGEN_BASE_PATH, SIZE_MOD_TEMPLATE,
    WEIGHT_MOD_TEMPLATE,
};
use std::fmt::Write as FmtWrite;
use std::fs::{create_dir, File};
use std::io::Write as IoWrite;
use std::path::PathBuf;

/// Binary that does all the codegen.
fn main() {
    // create the font weight modules for each supported font weight.
    for weight in FontWeight::variants() {
        let font_bytes = noto_font_by_weight(weight);
        codegen_font_weight_module(font_bytes, weight);
    }

    codegen_cargo_toml();
    codegen_lib_rs();
}

/// Generates the Cargo.toml with all relevant features.
fn codegen_cargo_toml() {
    // let mut path = PathBuf::from(CODEGEN_BASE_PATH);
    let mut path = PathBuf::from("..");
    path.push(format!("Cargo.toml"));

    let mut cargo_toml_file = File::options()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(path)
        .unwrap();

    let mut font_weight_features_str = String::new();
    FontWeight::variants().iter().for_each(|w| {
        writeln!(&mut font_weight_features_str, "{} = []", w.mod_name()).unwrap();
    });

    let mut font_size_features_str = String::new();
    BitmapHeight::variants().iter().copied().for_each(|size| {
        writeln!(&mut font_size_features_str, "size_{} = []", size.val()).unwrap();
    });

    let mut font_all_features_str = String::new();
    writeln!(&mut font_all_features_str, "all = [").unwrap();
    FontWeight::variants().iter().for_each(|w| {
        writeln!(&mut font_all_features_str, "    \"{}\",", w.mod_name()).unwrap();
    });
    BitmapHeight::variants().iter().for_each(|size| {
        writeln!(&mut font_all_features_str, "    \"size_{}\",", size.val()).unwrap();
    });
    writeln!(&mut font_all_features_str, "]").unwrap();

    // replace placeholders
    writeln!(
        &mut cargo_toml_file,
        "{}",
        CARGO_TOML_TEMPLATE
            .replace(
                "# %CODEGEN_FONT_WEIGHTS%",
                font_weight_features_str.as_str()
            )
            .replace("# %CODEGEN_FONT_SIZES%", font_size_features_str.as_str())
            .replace("# %CODEGEN_FEATURE_ALL%", font_all_features_str.as_str())
    )
    .unwrap();
}

/// Generates the lib.rs with all relevant features.
fn codegen_lib_rs() {
    let mut path = PathBuf::from(CODEGEN_BASE_PATH);
    path.push(format!("lib.rs"));

    let mut cargo_toml_file = File::options()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(path)
        .unwrap();

    // codegen font weight modules
    let mut weight_modules = String::new();
    {
        FontWeight::variants().iter().for_each(|w| {
            writeln!(&mut weight_modules, "mod {};", w.mod_name()).unwrap();
        });
    }

    // codegen font weight variants
    let mut weight_variants = String::new();
    {
        FontWeight::variants().iter().for_each(|w| {
            writeln!(
                &mut weight_variants,
                "    #[cfg(feature = \"{}\")]",
                w.mod_name()
            )
            .unwrap();
            writeln!(&mut weight_variants, "    {:?},", w).unwrap();
        });
    }

    // codegen font size enum variants
    let mut font_size_enum_variants = String::new();
    {
        BitmapHeight::variants().iter().for_each(|size| {
            writeln!(
                &mut font_size_enum_variants,
                "    #[cfg(feature = \"size_{}\")]",
                size.val()
            )
            .unwrap();
            writeln!(
                &mut font_size_enum_variants,
                "    Size{:?} = {},",
                size.val(),
                size.val()
            )
            .unwrap();
        });
    }

    // codegen get_bitmap match
    let mut get_bitmap_match = String::new();
    {
        FontWeight::variants().iter().for_each(|w| {
            writeln!(
                &mut get_bitmap_match,
                "        #[cfg(feature = \"{}\")]",
                w.mod_name()
            )
            .unwrap();
            writeln!(
                &mut get_bitmap_match,
                "        FontWeight::{:?} => match size.val() {{",
                w
            )
            .unwrap();
            BitmapHeight::variants().iter().for_each(|size| {
                writeln!(
                    &mut get_bitmap_match,
                    "            #[cfg(feature = \"size_{}\")]",
                    size.val()
                )
                .unwrap();
                writeln!(
                    &mut get_bitmap_match,
                    "            {} => crate::{}::size_{}::get_char(c),",
                    size.val(),
                    w.mod_name(),
                    size.val()
                )
                .unwrap();
            });

            writeln!(
                &mut get_bitmap_match,
                "            _ => panic!(\"unknown variant\"),"
            )
            .unwrap();
            writeln!(&mut get_bitmap_match, "        }}").unwrap();
        });
    }

    // codegen get_bitmap_width match
    let mut get_bitmap_width_match = String::new();
    {
        FontWeight::variants().iter().for_each(|w| {
            writeln!(
                &mut get_bitmap_width_match,
                "        #[cfg(feature = \"{}\")]",
                w.mod_name()
            )
            .unwrap();
            writeln!(
                &mut get_bitmap_width_match,
                "        FontWeight::{:?} => match size.val() {{",
                w
            )
            .unwrap();
            BitmapHeight::variants().iter().for_each(|size| {
                writeln!(
                    &mut get_bitmap_width_match,
                    "            #[cfg(feature = \"size_{}\")]",
                    size.val()
                )
                .unwrap();
                writeln!(
                    &mut get_bitmap_width_match,
                    "            {} => crate::{}::size_{}::BITMAP_WIDTH,",
                    size.val(),
                    w.mod_name(),
                    size.val()
                )
                .unwrap();
            });

            writeln!(
                &mut get_bitmap_width_match,
                "            _ => panic!(\"unknown variant\"),"
            )
            .unwrap();
            writeln!(&mut get_bitmap_width_match, "        }}").unwrap();
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
                    "// %CODEGEN_BITMAP_SIZE_ENUM_VARIANTS%",
                    font_size_enum_variants.as_str()
                )
                .replace("// %CODEGEN_GET_BITMAP%", get_bitmap_match.as_str())
                .replace("// %CODEGEN_GET_BITMAP_WIDTH%", get_bitmap_width_match.as_str())
        )
        .unwrap();
    }
}

/// Creates a font weight module, like `bold/mod.rs`.
fn codegen_font_weight_module(font_bytes: &[u8], weight: &FontWeight) {
    let mut mod_file_path = PathBuf::from(CODEGEN_BASE_PATH);
    mod_file_path.push(format!("{}", weight.mod_name()));
    // ignore error; dir might exist
    let _ = create_dir(&mod_file_path);
    mod_file_path.push(format!("mod.rs"));

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

    for size in BitmapHeight::variants().iter().map(|x| x.val()) {
        // add font modules for the font sizes
        writeln!(&mut mod_file, "#[cfg(feature = \"size_{}\")]", size).unwrap();
        writeln!(&mut mod_file, "pub mod size_{};", size).unwrap();

        let font = ToBitmapFont::new(size, font_bytes);
        codegen_font_weight_sub_modules(font, weight);
    }
}

/// Creates `bold/size_10.rs` etc. and adds it to `bold/mod.rs`.
fn codegen_font_weight_sub_modules(font: ToBitmapFont, weight: &FontWeight) {
    let mut mod_file_path = PathBuf::from(CODEGEN_BASE_PATH);
    mod_file_path.push(format!("{}", weight.mod_name()));
    mod_file_path.push(format!("size_{}.rs", font.bitmap_height()));

    let mut size_mod_file = File::options()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(mod_file_path)
        .unwrap();

    // replace placeholders
    writeln!(
        &mut size_mod_file,
        "{}",
        SIZE_MOD_TEMPLATE
            .replace("%FONT_WEIGHT%", weight.mod_name())
            .replace("%FONT_SIZE%", &format!("{}", font.bitmap_height()))
            .replace("%CODEGEN_BITMAP_HEIGHT%", &format!("{}", font.bitmap_height()))
            .replace("%CODEGEN_BITMAP_WIDTH%", &format!("{}", font.bitmap_width()))
    )
    .unwrap();

    // generate actual font bit map

    let mut code_range_string = String::new();
    writeln!(
        &mut code_range_string,
        "/// Returns the bitmap of the given character of the pre rendered\n\
        /// \"Noto Sans Mono\" raster for font weight {} and font size {}px",
        weight.mod_name(),
        font.font_size().round()
    )
    .unwrap();
    writeln!(&mut code_range_string, "#[inline]").unwrap();
    writeln!(
        &mut code_range_string,
        "pub const fn get_char(c: char) -> &'static [&'static [u8]] {{"
    )
    .unwrap();
    writeln!(&mut code_range_string, "    match c {{").unwrap();

    UnicodeIter::new()
        .filter(|s| s.is_char())
        .map(|s| s.get_char())
        .map(|char| (char, font.rasterize_to_bitmap(char)))
        .for_each(|(char, bitmap)| {
            writeln!(
                &mut code_range_string,
                "        // letter: '{}' / {:?}",
                char, char as usize as *const usize
            )
            .unwrap();

            if char == '\\' || char == '\'' {
                writeln!(&mut code_range_string, "        '\\{}' => &[", char).unwrap();
            } else {
                writeln!(&mut code_range_string, "        '{}' => &[", char).unwrap()
            }

            for row in bitmap {
                write!(&mut code_range_string, "            &[").unwrap();
                for (i, byte) in row.iter().enumerate() {
                    if i == row.len() - 1 {
                        write!(&mut code_range_string, "{}", byte).unwrap();
                    } else {
                        write!(&mut code_range_string, "{}, ", byte).unwrap();
                    }
                }
                writeln!(&mut code_range_string, "],").unwrap();
            }
            writeln!(&mut code_range_string, "        ],").unwrap();
        });
    writeln!(
        &mut code_range_string,
        "        _ => panic!(\"unsupported char\")"
    )
    .unwrap();
    // close match
    writeln!(&mut code_range_string, "    }}").unwrap();
    // close function
    writeln!(&mut code_range_string, "}}").unwrap();

    size_mod_file
        .write_all(code_range_string.as_bytes())
        .unwrap();
}
