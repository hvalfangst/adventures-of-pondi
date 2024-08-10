use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use minifb::{Key, KeyRepeat, Window};
use crate::state::command::CommandMap;
use crate::state::global_command::GlobalCommand;
use crate::state::{FRICTION, Obstacle};
use crate::state::player::Player;

pub fn handle_user_input(player: &mut Player, window: &mut Window, obstacles: &Vec<Obstacle>, commands: &CommandMap, global_commands: &HashMap<String, Rc<RefCell<dyn GlobalCommand>>>) {

    let legal_keys = [Key::Space, Key::D, Key::A, Key::X];

    // Flag to determine if any key was pressed
    let mut any_key_pressed = false;

    for key in legal_keys.iter() {
        if window.is_key_pressed(*key, KeyRepeat::Yes) {
            any_key_pressed = true;
            delegate_command(player, *key, &commands, obstacles);
        }
    }

    // Execute all global commands
    for (_, global_command) in global_commands.iter() {
        global_command.borrow().execute(player, obstacles);
    }

    // Apply friction to gradually slow down the player
    if !any_key_pressed {
        if player.vx > 0.0 {
            player.vx -= FRICTION;
            if player.vx < 0.0 {
                player.vx = 0.0;
            }
        } else if player.vx < 0.0 {
            player.vx += FRICTION;
            if player.vx > 0.0 {
                player.vx = 0.0;
            }
        }
    }
}

fn delegate_command(player: &mut Player, key: Key, commands: &CommandMap, obstacles: &Vec<Obstacle>) {
    if let Some(command) = commands.get(&key) {
        command.execute(player, obstacles);
    } else {
        println!("No command associated with key: {:?}", key);
    }
}