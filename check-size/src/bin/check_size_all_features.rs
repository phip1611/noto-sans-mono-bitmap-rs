use noto_sans_mono_bitmap::{RasterHeight, FontWeight};

// small program that uses the library and ensures that the compiled doesn't throw anything away.
// Thus, the library is definetely used and all code paths (=all input charactes) are valid.
// This way, I can determine the total usage in size.
fn main() {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    let char = line.chars().next().unwrap();

    let raster = noto_sans_mono_bitmap::get_raster(char, select_font_weigth(), select_font_size()).unwrap();
    println!("{raster:?}");
}

fn select_font_weigth() -> FontWeight {
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
