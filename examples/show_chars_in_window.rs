use minifb::{Key, Window, WindowOptions};
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

// Example that prints the chars in var "msg" to the screen using
// the pre-rendered bitmap font.
fn main() {
    // letters we want to print
    let msg = "Abc äöü!";
    let font_weight = FontWeight::Light;
    let bitmap_height = RasterHeight::Size64;
    let buffer_height = bitmap_height.val();
    let char_width = get_raster_width(font_weight, bitmap_height);
    let buffer_width = char_width * msg.chars().count();
    let mut draw_buffer = vec![0; buffer_height * buffer_width];

    // rasterize each char and draw it into the framebuffer
    for (char_i, char) in msg.chars().enumerate() {
        let bitmap_char = get_raster(char, font_weight, bitmap_height).expect("unknown char");
        for (row_i, row) in bitmap_char.bitmap().iter().enumerate() {
            for (col_i, intensity) in row.iter().enumerate() {
                let (r, g, b) = (*intensity as u32, *intensity as u32, *intensity as u32);
                let (r, g, b) = (255 - r, 255 - g, 255 - b);
                let rgb_32 = /*0 << 24 | */r << 16 | g << 8 | b;

                let index = char_i * bitmap_char.width() + col_i + row_i * buffer_width;

                draw_buffer[index] = rgb_32;
            }
        }
    }

    let mut window = Window::new(
        "Test - ESC to exit",
        buffer_width,
        buffer_height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&draw_buffer, buffer_width, buffer_height)
            .unwrap();
    }
}
