use std::thread;
use std::time::{Duration, Instant};

use minifb::{Key as key, Key, Window, WindowOptions};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;
use crate::{
    graphics::constants::*,
    graphics::sprites::*,
    state::{FRAME_DURATION, Player},
};
use crate::state::Obstacle;
use crate::state::utils::{draw_buffer, GameState, handle_key_presses, update_buffer_with_state};

pub fn start_event_loop(player: &mut Player, sprites: &Sprites, game_state: &mut Vec<Vec<GameState>>, obs: &Vec<Obstacle>) {
    // Create an event loop
    let event_loop = EventLoop::new();

    // Get the primary monitor dimensions
    let primary_monitor: MonitorHandle = event_loop.primary_monitor().expect("Failed to get primary monitor");
    let screen_size = primary_monitor.size();
    let screen_width: usize = screen_size.width as usize;
    let screen_height: usize = screen_size.height as usize;

    // Create a window with the dimensions of the primary monitor
    let mut window = Window::new(
        "Rust Platformer 0.1",
        screen_width as usize,
        screen_height as usize,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e); // Panic if window creation fails
    });

    // Initialize window buffer to store pixel data at low resolution
    let mut window_buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut scaled_buffer = vec![0; (screen_width * screen_height)];

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now(); // Record start time for frame timing

        // Handle user key presses to update state
        handle_key_presses(player, &mut window,  obs);

        // Update the pixel buffer with the current state visuals
        update_buffer_with_state(player, sprites, &mut window_buffer, game_state);

        // Scale the buffer to the screen resolution
        scale_buffer(&window_buffer, &mut scaled_buffer, WINDOW_WIDTH, WINDOW_HEIGHT, screen_width, screen_height);

        // Draw the scaled buffer onto the window
        window.update_with_buffer(&scaled_buffer, screen_width, screen_height).unwrap();

        // Maintain a frame rate of approximately 60 fps
        let elapsed = start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed);
        }
    }
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