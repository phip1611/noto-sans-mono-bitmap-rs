use noto_sans_mono_bitmap::{RasterHeight, FontWeight};

// small program that uses the library and ensures that the compiled doesn't throw anything away.
// Thus, the library is definetely used and all code paths (=all input charactes) are valid.
// This way, I can determine the total usage in size.
fn main() {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    let char = line.chars().next().unwrap();

    let raster = noto_sans_mono_bitmap::get_raster(char, FontWeight::Regular, RasterHeight::Size14).unwrap();
    println!("{raster:?}");
}
