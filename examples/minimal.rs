use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

// Minimal example.
fn main() {
    let width = get_raster_width(FontWeight::Regular, RasterHeight::Size14);
    println!(
        "Each char of the mono-spaced font will be {}px in width if the font \
         weight={:?} and the bitmap height={}",
        width,
        FontWeight::Regular,
        RasterHeight::Size14.val()
    );
    let bitmap_char =
        get_raster('A', FontWeight::Regular, RasterHeight::Size14).expect("unsupported char");
    println!("{:?}", bitmap_char);
    for (row_i, row) in bitmap_char.bitmap().iter().enumerate() {
        for (col_i, pixel) in row.iter().enumerate() {
            println!("[{:02}][{:02}]: {:03}", row_i, col_i, pixel);
        }
    }
}
