use minifb::{Key, Window, WindowOptions};
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_PIXELS: usize = HEIGHT * WIDTH;

// This example draws some text approximately in the middle of the window
// The background colour of the pixels is taken into account to so that the text is effectively
// drawn with no solid background

fn main() {
    let mut draw_buffer = vec![0; NUM_PIXELS];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let fill_colour = (60 << 16) + (60 << 8) + 60; // This is a nice grey colour

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the window by filling the buffer with the fill colour
        draw_buffer[0..NUM_PIXELS].fill(fill_colour);

        let msg = "Hello World";
        draw_string(
            msg,
            400,
            300,
            FontWeight::Regular,
            RasterHeight::Size16,
            &mut draw_buffer,
        );

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&draw_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn draw_string(
    msg: &str,
    x: u32,
    y: u32,
    font_weight: FontWeight,
    raster_height: RasterHeight,
    draw_buffer: &mut [u32],
) {
    for (char_i, char) in msg.chars().enumerate() {
        let char_raster = get_raster(char, font_weight, raster_height).expect("unknown char");
        for (row_i, row) in char_raster.raster().iter().enumerate() {
            for (col_i, intensity) in row.iter().enumerate() {
                //Figure out the index of the pixel to update
                let index = char_i * char_raster.width()
                    + col_i
                    + row_i * WIDTH
                    + (x as usize)
                    + (y as usize * WIDTH);

                //Sample the pixel, as we need to add the intensity to it
                let curr_pixel_rgb = draw_buffer[index];

                //Split into RGB so that we can add the intensity to each component separately
                let mut r = ((curr_pixel_rgb & 0xff0000) >> 16) as u8;
                let mut g = ((curr_pixel_rgb & 0xff00) >> 8) as u8;
                let mut b = (curr_pixel_rgb & 0xff) as u8;

                //Use a saturating add to clamp to max u8 value
                r = r.saturating_add(*intensity);
                g = g.saturating_add(*intensity);
                b = b.saturating_add(*intensity);

                let new_pixel_rgb = ((r as u32) << 16) + ((g as u32) << 8) + (b as u32);

                draw_buffer[index] = new_pixel_rgb;
            }
        }
    }
}
