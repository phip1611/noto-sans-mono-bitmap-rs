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

    let raster = noto_sans_mono_bitmap::get_raster(char, select_font_weight(), select_font_size()).unwrap();
    println!("{raster:?}");
}

fn select_font_weight() -> FontWeight {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    let char = line.chars().next().unwrap();

    match char {
        'a' => FontWeight::Light,
        'b' => FontWeight::Regular,
        'c' => FontWeight::Bold,
        _ => panic!()
    }
}

fn select_font_size() -> RasterHeight {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    let char = line.chars().next().unwrap();

    match char {
        'a' => RasterHeight::Size16,
        'c' => RasterHeight::Size20,
        'e' => RasterHeight::Size24,
        'f' => RasterHeight::Size32,
        _ => panic!()
    }
}
