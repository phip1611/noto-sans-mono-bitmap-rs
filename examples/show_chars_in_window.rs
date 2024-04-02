use minifb::{Key, Window, WindowOptions};
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

const MSG: &str = "Abc Hello world 123!_-:.qg Äöü#0ß`���� wwwWWW";
const LINES_TO_PRINT: usize = 5;

const FONT_WEIGHT: FontWeight = FontWeight::Regular;
const RASTER_HEIGHT: RasterHeight = RasterHeight::Size32;

// Example that some characters into a window using the font characteristics of
// the pre-rasterized characters.
fn main() {
    let buffer_height = RASTER_HEIGHT.val() * LINES_TO_PRINT;
    let char_width = get_raster_width(FONT_WEIGHT, RASTER_HEIGHT);
    let buffer_width = char_width * MSG.chars().count();
    let mut draw_buffer = vec![0; (buffer_height) * buffer_width];

    // rasterize each char and draw it into the framebuffer
    for i in 0..LINES_TO_PRINT {
        print_msg(i, buffer_width, &mut draw_buffer);
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

fn print_msg(line: usize, buffer_width: usize, draw_buffer: &mut [u32]) {
    for (char_i, char) in MSG.chars().enumerate() {
        let char_raster = get_raster(char, FONT_WEIGHT, RASTER_HEIGHT).expect("unknown char");
        let line_offset = line * char_raster.height();
        for (row_i, row) in char_raster.raster().iter().enumerate() {
            for (col_i, intensity) in row.iter().enumerate() {
                let (r, g, b) = (*intensity as u32, *intensity as u32, *intensity as u32);
                let (r, g, b) = (255 - r, 255 - g, 255 - b);
                let rgb_32 = /*0 << 24 | */r << 16 | g << 8 | b;

                let index =
                    char_i * char_raster.width() + col_i + (line_offset + row_i) * buffer_width;

                draw_buffer[index] = rgb_32;
            }
        }
    }
}
