use noto_sans_mono_bitmap::{RasterHeight, FontWeight};

// Small program that uses the library and ensures that the compiler doesn't
// throw anything away that should be there.
// Thus, the library is definitely used and all inputs/all expected paths are
// possible during runtime. This way, we can determine the total size when
// compiled.
fn main() {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    let char = line.chars().next().unwrap();

    let raster = noto_sans_mono_bitmap::get_raster(char, FontWeight::Regular, RasterHeight::Size16).unwrap();
    println!("{raster:?}");
}
