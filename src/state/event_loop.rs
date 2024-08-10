use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::Instant;

use minifb::{Key, Window, WindowOptions};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;

use crate::{graphics::constants::*, graphics::sprites::*, state::{FRAME_DURATION}};
use crate::graphics::renderer::render;
use crate::state::{BACKGROUND_CHANGE_INTERVAL, Obstacle};
use crate::state::command::{CommandMap};
use crate::state::global_command::GlobalCommand;
use crate::state::input_handler::handle_user_input;
use crate::state::player::Player;
use crate::state::update::{update};

pub fn start_event_loop(player: &mut Player, sprites: &Sprites, obstacles: &Vec<Obstacle>, fullscreen: bool, commands: CommandMap, global_commands: HashMap<String, Rc<RefCell<dyn GlobalCommand>>>) {

    // Determine window size based on fullscreen flag
    let (window_width, window_height) = if fullscreen {
        let primary_monitor: MonitorHandle =  EventLoop::new().primary_monitor().expect("Failed to get primary monitor");
        let screen_size = primary_monitor.size();
        (screen_size.width as usize, screen_size.height as usize)
    } else {
        (SCALED_WINDOW_WIDTH, SCALED_WINDOW_HEIGHT)
    };

    // Create a window with the dimensions of the primary monitor
    let mut window = Window::new(
        "Rust Platformer 0.1",
        window_width,
        window_height,
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

        // Update game state based on user input
        handle_user_input(player, &mut window, &obstacles, &commands, &global_commands);

        // Check if it's time to change the background sprite
        if last_background_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL {
            background_sprite_index = (background_sprite_index + 1) % 4; // Cycle between 0 and 3
            last_background_change = Instant::now(); // Reset the timer
        }

        // Update the pixel buffer with the current game state
        update(player, sprites, &mut window_buffer, background_sprite_index);

        // Render the updated buffer
        render(window_width, window_height, &mut window, &mut window_buffer, &mut scaled_buffer);

        // Maintain a frame rate of approximately 60 fps
        let elapsed = start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed);
        }
    }
}