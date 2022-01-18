//! Helper binary to rasterize a few letters with fontdue in a window and display its
//! rasterized dimensions.

use codegen::font::ToBitmapFont;
use minifb::{Key, Window, WindowOptions};

fn main() {
    // letters we want to print
    let msg = "ÄyA-#+`?ß§";

    let font_bytes = include_bytes!("../res/NotoSansMono-Regular.ttf") as &[u8];

    let bitmap_height = 36;
    let font = ToBitmapFont::new(bitmap_height, font_bytes);
    let height = font.bitmap_height();
    // make sure N mono-sized letters can be inside the frame buffer
    let width = font.bitmap_width() * msg.chars().count();
    let mut draw_buffer: Vec<u32> = vec![0; width * height];

    // set white background
    for i in draw_buffer.iter_mut() {
        let (r, g, b) = (255, 255, 255);
        let rgb_32 = /*0 << 24 | */r << 16 | g << 8 | b;
        *i = rgb_32;
    }

    // rasterize each char and draw it into the framebuffer
    for (char_i, char) in msg.chars().enumerate() {
        let letter_bitmap = font.rasterize_to_bitmap(char);
        for (row_i, row) in letter_bitmap.iter().enumerate() {
            for (col_i, intensity) in row.iter().enumerate() {
                let (r, g, b) = (*intensity as u32, *intensity as u32, *intensity as u32);
                let (r, g, b) = (255 - r, 255 - g, 255 - b);
                let rgb_32 = /*0 << 24 | */r << 16 | g << 8 | b;

                let index = char_i * font.bitmap_width() + col_i + row_i * width;

                draw_buffer[index] = rgb_32;
            }
        }
    }

    // draw red borders for 3x1 grid
    for border_i in 1..msg.chars().count() {
        let (r, g, b) = (255, 0, 0);
        let rgb_32 = /*0 << 24 |*/ r << 16 | g << 8 | b;
        for j in 0..height {
            let index = border_i * font.bitmap_width() + j * width;
            draw_buffer[index] = rgb_32;
        }
    }

    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_millis(1000 / 60)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&draw_buffer, width, height)
            .unwrap();
    }
}
