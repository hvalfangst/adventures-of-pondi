use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use minifb::{Key, KeyRepeat};

use crate::state::{apply_friction, Context};
use crate::state::command::CommandMap;
use crate::state::global_command::GlobalCommand;

pub fn handle_user_input(context: &mut Context, commands: &CommandMap, global_commands: &HashMap<String, Rc<RefCell<dyn GlobalCommand>>>) {

    let legal_keys = [Key::Space, Key::D, Key::A, Key::X];

    // Flag to determine if any key was pressed
    let mut any_key_pressed = false;

    for key in legal_keys.iter() {
        if context.window.is_key_pressed(*key, KeyRepeat::Yes) {
            any_key_pressed = true;
            delegate_command(*key, &commands, context);
        }
    }

    // Execute all global commands
    for (_, global_command) in global_commands.iter() {
        global_command.borrow().execute(context);
    }

    // Apply friction to gradually slow down the player
    if !any_key_pressed {
        apply_friction(context);
    }
}

fn delegate_command(key: Key, commands: &CommandMap, context: &mut Context) {
    if let Some(command) = commands.get(&key) {
        command.execute(context);
    } else {
        println!("No command associated with key: {:?}", key);
    }
}