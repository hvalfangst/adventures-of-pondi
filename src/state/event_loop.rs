use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::Instant;

use minifb::Key;

use crate::graphics::renderer::render;
use crate::state::{BACKGROUND_CHANGE_INTERVAL, GameState};
use crate::state::command::CommandMap;
use crate::state::FRAME_DURATION;
use crate::state::global_command::GlobalCommand;
use crate::state::input_handler::handle_user_input;
use crate::state::update::update;

pub fn start_event_loop(mut game_state: GameState, commands: CommandMap, global_commands: HashMap<String, Rc<RefCell<dyn GlobalCommand>>>, sink: &mut rodio::Sink) {



    // Variables for background sprite changing
    let mut last_grass_sprite_index_change = Instant::now();
    let mut last_sky_sprite_index_change = Instant::now();
    let mut last_footstep_time = Instant::now();

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while game_state.window.is_open() && !game_state.window.is_key_down(Key::Escape) {
        let start = Instant::now();

        if last_footstep_time.elapsed() >= std::time::Duration::from_millis(500) {
            game_state.footstep_active = true;
            last_footstep_time = Instant::now();
        }


        handle_user_input(&mut game_state, &commands, &global_commands, sink);

        // Change grass sprite every second - alternate between 0 and 1
        if last_grass_sprite_index_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL {
            game_state.grass_sprite_index = (game_state.grass_sprite_index + 1) % 2; // Cycle between 0 and 1
            last_grass_sprite_index_change = Instant::now(); // Reset the timer to current time
        }

        // Change sky sprite every 2 seconds - alternate between 0 and 3
        if last_sky_sprite_index_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL * 2 {
            game_state.sky_sprite_index = (game_state.sky_sprite_index + 1) % 4; // Cycle between 0 and 3
            last_sky_sprite_index_change = Instant::now(); // Reset the timer to current time
        }

        // Update the pixel buffer with the current game state
        update(&mut game_state);

        // Render the updated buffer
        render(&mut game_state);

        // Maintain a frame rate of 60 fps
        let elapsed = start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed);
        }
    }
}