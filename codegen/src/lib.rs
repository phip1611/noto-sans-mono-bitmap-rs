//! Codegen utilities for the main codegen binary and examples/sub binaries in `src/bin`.

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
pub const SIZE_MOD_TEMPLATE: &str = include_str!("codegen_templates/size_mod.template.txt");
/// Used as template to generate Rust modules for a specific font weight.
pub const WEIGHT_MOD_TEMPLATE: &str = include_str!("codegen_templates/weight_mod.template.txt");
/// Used as template to generate the Cargo.toml.
pub const CARGO_TOML_TEMPLATE: &str = include_str!("codegen_templates/Cargo.toml.txt");
/// Used as template to generate the lib.rs.
pub const CARGO_LIB_RS: &str = include_str!("codegen_templates/lib.rs.template.txt");

/// Path into the main repository, where the codegen manipulates files.
pub const CODEGEN_BASE_PATH: &str = "../src/";
