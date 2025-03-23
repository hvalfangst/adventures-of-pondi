use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use minifb::{Key, KeyRepeat};
use rodio::Sink;
use crate::state::{apply_friction, GameState};
use crate::state::command::CommandMap;
use crate::state::global_command::GlobalCommand;

pub fn handle_user_input(game_state: &mut GameState, commands: &CommandMap, global_commands: &HashMap<String, Rc<RefCell<dyn GlobalCommand>>>, sink: &mut Sink) {

    let legal_keys = [Key::Space, Key::D, Key::A, Key::X];

    // Flag to determine if any key was pressed
    let mut any_key_pressed = false;

    for key in legal_keys.iter() {
        if game_state.window.is_key_pressed(*key, KeyRepeat::Yes) {
            any_key_pressed = true;
            delegate_command(*key, &commands, game_state, sink);
        }
    }

    // Execute all global commands
    for (_, global_command) in global_commands.iter() {
        global_command.borrow().execute(game_state, sink);
    }

    // Apply friction to gradually slow down the player
    if !any_key_pressed {
        apply_friction(game_state);
        sink.stop()
    }
}

fn delegate_command(key: Key, commands: &CommandMap, game_state: &mut GameState, sink: &mut Sink) {
    if let Some(command) = commands.get(&key) {
        command.execute(game_state, sink);
    } else {
        println!("No command associated with key: {:?}", key);
    }
}