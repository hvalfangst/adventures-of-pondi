use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::Instant;

use minifb::Key;

use crate::graphics::renderer::render;
use crate::state::{BACKGROUND_CHANGE_INTERVAL, Context};
use crate::state::command::CommandMap;
use crate::state::FRAME_DURATION;
use crate::state::global_command::GlobalCommand;
use crate::state::input_handler::handle_user_input;
use crate::state::update::update;

pub fn start_event_loop(mut context: Context, commands: CommandMap, global_commands: HashMap<String, Rc<RefCell<dyn GlobalCommand>>>) {



    // Variables for background sprite changing
    let mut last_grass_sprite_index_change = Instant::now();
    let mut last_sky_sprite_index_change = Instant::now();

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while context.window.is_open() && !context.window.is_key_down(Key::Escape) {
        let start = Instant::now();


        handle_user_input(&mut context, &commands, &global_commands);

        // Change grass sprite every second - alternate between 0 and 1
        if last_grass_sprite_index_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL {
            context.grass_sprite_index = (context.grass_sprite_index + 1) % 2; // Cycle between 0 and 1
            last_grass_sprite_index_change = Instant::now(); // Reset the timer to current time
        }

        // Change sky sprite every 2 seconds - alternate between 0 and 3
        if last_sky_sprite_index_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL * 2 {
            context.sky_sprite_index = (context.sky_sprite_index + 1) % 4; // Cycle between 0 and 3
            last_sky_sprite_index_change = Instant::now(); // Reset the timer to current time
        }

        // Update the pixel buffer with the current game state
        update(&mut context);

        // Render the updated buffer
        render(&mut context);

        // Maintain a frame rate of 60 fps
        let elapsed = start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed);
        }
    }
}