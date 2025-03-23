use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::rc::Rc;
use std::thread::sleep;
use rodio::{Sink, Source};
use crate::graphics::renderer::render;
use crate::state::{GameState, Direction, GRAVITY, GROUND, jump_obstacles, LOWER_BOUND, Obstacle, UPPER_BOUND, DOWN_SOUND};
use crate::state::player::Player;
use crate::state::update::update;
pub trait GlobalCommand {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub struct ApplyGravity;

impl GlobalCommand for ApplyGravity {
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

impl GlobalCommand for JumpingObstacles {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        jump_obstacles(game_state, sink);
    }
}

pub struct VerticalBounds;

impl GlobalCommand for VerticalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Prevent the player from moving out vertical (y) bounds
        if game_state.player.y <= 40.0 {
            game_state.player.on_ground = false;
            game_state.player.y = GROUND;
        }
    }
}

pub struct HorizontalBounds;

impl GlobalCommand for HorizontalBounds {
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

impl GlobalCommand for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.game_over {
            println!("Game Over!");

            for _ in 0..4 {
                update(game_state);
                render(game_state);
                game_state.game_over_index += 1;
                sleep(std::time::Duration::from_millis(200));
            }

            game_state.game_over_index = 0;
            game_state.player = Player::new(0.0, GROUND); // Reset player state
        }
    }
}

pub struct ApplyFriction;

impl GlobalCommand for ApplyFriction {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.direction == Direction::Left {
            game_state.player.x -= game_state.player.vx;
        } else {
            game_state.player.x += game_state.player.vx;
        }
        game_state.player.y += game_state.player.vy;
    }
}

pub fn initialize_global_command_map() -> HashMap<String, Rc<RefCell<dyn GlobalCommand>>> {
    let mut global_commands: HashMap<String, Rc<RefCell<dyn GlobalCommand>>> = HashMap::new();
    global_commands.insert("JumpingObstacles".to_string(), Rc::new(RefCell::new(JumpingObstacles)));
    global_commands.insert("ApplyGravity".to_string(), Rc::new(RefCell::new(ApplyGravity)));
    global_commands.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(VerticalBounds)));
    global_commands.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(HorizontalBounds)));
    global_commands.insert("CheckGameOver".to_string(), Rc::new(RefCell::new(CheckGameOver)));
    global_commands.insert("ApplyFriction".to_string(), Rc::new(RefCell::new(ApplyFriction)));



    global_commands
}