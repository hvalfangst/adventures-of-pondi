use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::rc::Rc;
use std::thread::sleep;
use crate::state::{apply_friction, jump_obstacles, Direction, GameState, DOWN_SOUND, GRAVITY, GROUND, LOWER_BOUND, UPPER_BOUND};
use rodio::{Sink, Source};
use crate::graphics::renderer::render_pixel_buffer;
use crate::state::player::Player;
use crate::state::update::update_pixel_buffer;


pub fn execute_core_logic(game_state: &mut GameState, global_commands: &HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut Sink, any_key_pressed: bool) {
    for (_, global_command) in global_commands.iter() {
        global_command.borrow().execute(game_state, sink);
    }

    if !any_key_pressed {
        apply_friction(game_state);
        sink.stop();
    }
}

pub trait CoreLogic {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub struct ApplyGravity;

impl CoreLogic for ApplyGravity {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Apply gravity to the player
        if !game_state.player.on_ground && !game_state.player.on_obstacle {
            game_state.player.vy += GRAVITY;
        }

        let mut obstacle_landed = false;

        // Apply gravity to all obstacles which have falling boolean
        for obstacle in game_state.all_maps[game_state.current_map_index].obstacles.iter_mut() {
            if obstacle.active && obstacle.falling {
                if obstacle.velocity_y >= 16.0 {
                    // println!("obstacle.velocity_y: {}", obstacle.velocity_y);
                    obstacle_landed = true;
                    obstacle.falling = false;
                } else {
                    obstacle.y_bottom += GRAVITY * 3.0;
                    obstacle.y_top += GRAVITY * 3.0;
                    obstacle.velocity_y += GRAVITY * 3.0;
                    // println!("obstacle.y_bottom: {}, obstacle.y_top: {}", obstacle.y_bottom, obstacle.y_top);
                }
            }
        }

        if obstacle_landed {
            let file = &game_state.sounds[DOWN_SOUND]; // Get the raw sound data (Vec<u8>)
            let cursor = Cursor::new(file.clone()); // Clone to create an owned Cursor<Vec<u8>>

            let source = rodio::Decoder::new(BufReader::new(cursor))
                .unwrap()
                .take_duration(std::time::Duration::from_millis(1000));

            sink.append(source); // Play the sound

            // TODO
            // Sort obstacles by DESC by y_bottom, meaning the highest obstacles will be put first in the vector (due to polar coordinates)
            game_state.all_maps[game_state.current_map_index].obstacles.sort_by(|a, b| a.y_bottom.partial_cmp(&b.y_bottom).unwrap());
        }
    }
}

pub struct JumpingObstacles;

impl CoreLogic for JumpingObstacles {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        jump_obstacles(game_state, sink);
    }
}

pub struct VerticalBounds;

impl CoreLogic for VerticalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Prevent the player from moving out vertical (y) bounds
        if game_state.player.y <= 40.0 {
            game_state.player.on_ground = false;
            game_state.player.y = GROUND;
        }
    }
}

pub struct HorizontalBounds;

impl CoreLogic for HorizontalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Prevent the player from moving out horizontal (x) bounds
        if game_state.player.x < LOWER_BOUND {
            game_state.player.x = LOWER_BOUND;
            game_state.player.vx = 0.0;
        } else if game_state.player.x >= UPPER_BOUND {
            game_state.player.x = 0.0;
            game_state.player.vx = 0.0;
            game_state.current_map_index += 1
        }
    }
}

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.game_over {
            println!("Game Over!");

            for _ in 0..4 {
                update_pixel_buffer(game_state);
                render_pixel_buffer(game_state);
                game_state.game_over_index += 1;
                sleep(std::time::Duration::from_millis(200));
            }

            game_state.game_over_index = 0;
            game_state.player = Player::new(0.0, GROUND); // Reset player state
        }
    }
}

pub struct ApplyFriction;

impl CoreLogic for ApplyFriction {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.direction == Direction::Left {
            game_state.player.x -= game_state.player.vx;
        } else {
            game_state.player.x += game_state.player.vx;
        }
        game_state.player.y += game_state.player.vy;
    }
}

pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();
    logic_map.insert("JumpingObstacles".to_string(), Rc::new(RefCell::new(JumpingObstacles)));
    logic_map.insert("ApplyGravity".to_string(), Rc::new(RefCell::new(ApplyGravity)));
    logic_map.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(VerticalBounds)));
    logic_map.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(HorizontalBounds)));
    logic_map.insert("CheckGameOver".to_string(), Rc::new(RefCell::new(CheckGameOver)));
    logic_map.insert("ApplyFriction".to_string(), Rc::new(RefCell::new(ApplyFriction)));

    logic_map
}
