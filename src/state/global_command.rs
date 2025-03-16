use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufReader;
use std::rc::Rc;
use std::thread::sleep;
use rodio::{Sink, Source};
use crate::graphics::renderer::render;
use crate::state::{Context, Direction, GRAVITY, GROUND, jump_obstacles, LOWER_BOUND, Obstacle, UPPER_BOUND};
use crate::state::player::Player;
use crate::state::update::update;
pub trait GlobalCommand {
    fn execute(&self, context: &mut Context, sink: &mut Sink);
}

pub struct ApplyGravity;

impl GlobalCommand for ApplyGravity {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        // Apply gravity to the player
        if !context.player.on_ground && !context.player.on_obstacle {
            context.player.vy += GRAVITY;
        }

        let mut obstacle_landed = false;

        // Apply gravity to all obstacles which have falling boolean
        for obstacle in context.all_maps[context.current_map_index].obstacles.iter_mut() {
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
            let file = std::fs::File::open("down.wav").unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap().take_duration(std::time::Duration::from_millis(1000));

            // Append the sound source to the audio sink for playback
            let _result = sink.append(source);
            // Sort obstacles by DESC by y_bottom, meaning the highest obstacles will be put first in the vector (due to polar coordinates)
            context.all_maps[context.current_map_index].obstacles.sort_by(|a, b| a.y_bottom.partial_cmp(&b.y_bottom).unwrap());
        }
    }
}

pub struct JumpingObstacles;

impl GlobalCommand for JumpingObstacles {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        jump_obstacles(context, sink);
    }
}

pub struct VerticalBounds;

impl GlobalCommand for VerticalBounds {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        // Prevent the player from moving out vertical (y) bounds
        if context.player.y <= 40.0 {
            context.player.on_ground = false;
            context.player.y = GROUND;
        }
    }
}

pub struct HorizontalBounds;

impl GlobalCommand for HorizontalBounds {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        // Prevent the player from moving out horizontal (x) bounds
        if context.player.x < LOWER_BOUND {
            context.player.x = LOWER_BOUND;
            context.player.vx = 0.0;
        } else if context.player.x >= UPPER_BOUND {
            context.player.x = 0.0;
            context.player.vx = 0.0;
            context.current_map_index += 1
        }
    }
}

pub struct CheckGameOver;

impl GlobalCommand for CheckGameOver {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        if context.player.game_over {
            println!("Game Over!");

            for _ in 0..4 {
                update(context);
                render(context);
                context.game_over_index += 1;
                sleep(std::time::Duration::from_millis(200));
            }

            context.game_over_index = 0;
            context.player = Player::new(0.0, GROUND); // Reset player state
        }
    }
}

pub struct ApplyFriction;

impl GlobalCommand for ApplyFriction {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        if context.player.direction == Direction::Left {
            context.player.x -= context.player.vx;
        } else {
            context.player.x += context.player.vx;
        }
        context.player.y += context.player.vy;
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