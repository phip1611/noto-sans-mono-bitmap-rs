use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapHeight, FontWeight};

// Minimal example.
fn main() {
    let width = get_bitmap_width(FontWeight::Regular, BitmapHeight::Size16);
    println!(
        "Each char of the mono-spaced font will be {}px in width if the font \
         weight={:?} and the bitmap height={}",
        width,
        FontWeight::Regular,
        BitmapHeight::Size16.val()
    );
    let bitmap_char = get_bitmap('A', FontWeight::Regular, BitmapHeight::Size16);
    println!("{:?}", bitmap_char);
    for (row_i, row) in bitmap_char.bitmap().iter().enumerate() {
        for (col_i, pixel) in row.iter().enumerate() {
            println!("[{:02}][{:02}]: {:03}", row_i, col_i, pixel);
        }
    }
}
