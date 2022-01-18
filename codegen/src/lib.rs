//! Codegen utilities for the main codegen binary and examples/sub binaries in `src/bin`.

pub mod font;
pub mod unicode;

/// The height of the bitmap for the font rasterization.
/// The width will be a little bit less. Furthermore, the
/// font size is also less than the bitmap height, because
/// there will be a small vertical padding to the top and
/// the bottom for proper alignment of characters.
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum BitmapHeight {
    Size14 = 14,
    Size16 = 16,
    Size18 = 18,
    Size20 = 20,
    Size22 = 22,
    Size24 = 24,
    Size32 = 32,
    Size64 = 64,
}

impl BitmapHeight {
    pub const fn val(self) -> usize {
        self as _
    }

    pub const fn variants() -> &'static [Self] {
        &[
            Self::Size14,
            Self::Size16,
            Self::Size18,
            Self::Size20,
            Self::Size22,
            Self::Size24,
            Self::Size32,
            Self::Size64,
        ]
    }
}

/// Used as template to generate Rust modules for a specific font size.
pub const SIZE_MOD_TEMPLATE: &'static str = include_str!("codegen_templates/size_mod.template.txt");
/// Used as template to generate Rust modules for a specific font weight.
pub const WEIGHT_MOD_TEMPLATE: &'static str =
    include_str!("codegen_templates/weight_mod.template.txt");
/// Used as template to generate the Cargo.toml.
pub const CARGO_TOML_TEMPLATE: &'static str = include_str!("codegen_templates/Cargo.toml.txt");
/// Used as template to generate the lib.rs.
pub const CARGO_LIB_RS: &'static str = include_str!("codegen_templates/lib.rs.template.txt");

/// Path into the main repository, where the codegen manipulates files.
pub const CODEGEN_BASE_PATH: &'static str = "../src/";
