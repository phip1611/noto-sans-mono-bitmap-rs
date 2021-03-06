//! Module for letters with the font weight bold.
//!
//! The bitmap fonts contains all unicode symbols from 0x00 to 0x17f with
//! the exception of control characters. It includes Basic Latin
//! Latin-1 Supplement and Latin extended A. This means the typical letters
//! and symbols from a QWERTZ/QWERTY keyboard plus symbols such as Ö, Ä,
//! and Ü.

#[cfg(feature = "size_14")]
pub mod size_14;
#[cfg(feature = "size_16")]
pub mod size_16;
#[cfg(feature = "size_18")]
pub mod size_18;
#[cfg(feature = "size_20")]
pub mod size_20;
#[cfg(feature = "size_22")]
pub mod size_22;
#[cfg(feature = "size_24")]
pub mod size_24;
#[cfg(feature = "size_32")]
pub mod size_32;
#[cfg(feature = "size_64")]
pub mod size_64;
