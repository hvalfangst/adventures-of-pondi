use std::thread;
use std::time::Instant;

use minifb::{Key, Window, WindowOptions};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;

use crate::{
    graphics::constants::*,
    graphics::sprites::*,
    state::{FRAME_DURATION, Player},
};
use crate::state::{BACKGROUND_CHANGE_INTERVAL, Obstacle};
use crate::state::utils::{handle_key_presses, update_buffer_with_state};

pub fn start_event_loop(player: &mut Player, sprites: &Sprites, obs: &Vec<Obstacle>, fullscreen: bool) {
    // Create an event loop
    let event_loop = EventLoop::new();

    // Determine window size based on fullscreen flag
    let (window_width, window_height) = if fullscreen {
        let primary_monitor: MonitorHandle = event_loop.primary_monitor().expect("Failed to get primary monitor");
        let screen_size = primary_monitor.size();
        (screen_size.width as usize, screen_size.height as usize)
    } else {
        (SCALED_WINDOW_WIDTH, SCALED_WINDOW_HEIGHT)
    };

    // Create a window with the desired dimensions
    // Create a window with the dimensions of the primary monitor
    let mut window = Window::new(
        "Rust Platformer 0.1",
        window_width as usize,
        window_width as usize,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e); // Panic if window creation fails
    });

    // Initialize window buffer to store pixel data at low resolution
    let mut window_buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut scaled_buffer = vec![0; window_width * window_height];

    // Variables for background sprite changing
    let mut background_sprite_index = 0;
    let mut last_background_change = Instant::now();

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now(); // Record start time for frame timing

        // Handle user key presses to update state
        handle_key_presses(player, &mut window,  obs);

        // Check if it's time to change the background sprite
        if last_background_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL {
            background_sprite_index = (background_sprite_index + 1) % 4; // Cycle between 0 and 3
            last_background_change = Instant::now(); // Reset the timer
        }

        // Update the pixel buffer with the current state visuals
        update_buffer_with_state(player, sprites, &mut window_buffer, background_sprite_index);

        // Scale the buffer to the screen resolution
        scale_buffer(&window_buffer, &mut scaled_buffer, WINDOW_WIDTH, WINDOW_HEIGHT, window_width, window_height);

        // Draw the scaled buffer onto the window
        window.update_with_buffer(&scaled_buffer, window_width, window_height).unwrap();

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