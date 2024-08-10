use minifb::Window;
use crate::graphics::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render(window_width: usize, window_height: usize, window: &mut Window, window_buffer: &mut Vec<u32>, mut scaled_buffer: &mut Vec<u32>) {
    // Scale the buffer to the screen resolution
    scale_buffer(&window_buffer, &mut scaled_buffer, WINDOW_WIDTH, WINDOW_HEIGHT, window_width, window_height);

    // Draw the scaled buffer onto the window
    window.update_with_buffer(&scaled_buffer, window_width, window_height).unwrap();
}

// Function to scale a buffer to a different resolution
fn scale_buffer(src: &[u32], dst: &mut [u32], src_width: usize, src_height: usize, dst_width: usize, dst_height: usize) {
    let x_ratio = src_width as f32 / dst_width as f32;
    let y_ratio = src_height as f32 / dst_height as f32;

    for y in 0..dst_height {
        for x in 0..dst_width {
            let src_x = (x as f32 * x_ratio).floor() as usize;
            let src_y = (y as f32 * y_ratio).floor() as usize;
            dst[y * dst_width + x] = src[src_y * src_width + src_x];
        }
    }
}