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

pub use crate::unicode::SUPPORTED_UNICODE_RANGES;

pub mod bytes_outsourcer;
pub mod font;
pub mod unicode;

// All supported raster heights that will be generated.
pub const SUPPORTED_RASTER_HEIGHTS: &[RasterHeight] = &[
    RasterHeight::new(14, true),
    RasterHeight::new(16, false),
    RasterHeight::new(18, false),
    RasterHeight::new(20, false),
    RasterHeight::new(22, false),
    RasterHeight::new(24, false),
    RasterHeight::new(32, false),
    RasterHeight::new(64, false),
];

/// Teight of the rasterization process of certain characters. Like the font
/// size but this describes the size of the outer box. Hence, the font size
/// is a little smaller.
///
/// The width of each character is influenced by this property. It will be a
/// little less as well
#[derive(Debug, Clone, Copy)]
pub struct RasterHeight {
    value: u32,
    // if the feature is included by default in Cargo.toml
    default_feature: bool,
}

impl RasterHeight {
    const fn new(value: u32, default_feature: bool) -> Self {
        Self {
            value,
            default_feature,
        }
    }

    pub const fn value(self) -> u32 {
        self.value
    }

    pub fn feature_name(&self) -> String {
        format!("size_{}", self.value)
    }

    pub fn default_feature(&self) -> bool {
        self.default_feature
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
/// Contains the rasterized bytes of all characters.
pub const CODEGEN_RASTERIZED_BYTES_PATH: &str = "../src/res_rasterized_characters";

/*#[cfg(test)]
mod tests {
    use super::*;

}*/
