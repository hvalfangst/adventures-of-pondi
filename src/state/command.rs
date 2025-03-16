use std::collections::HashMap;
use std::io::BufReader;
use std::sync::Arc;

use minifb::Key;
use rodio::{Sink, Source};
use crate::graphics::sprites::Sprites;
use crate::sort_obstacles_by_y;
use crate::state::{ACCELERATION, Context, JUMP_VELOCITY, MAX_VELOCITY, Obstacle, remove_box};
use crate::state::Direction::{Left, Right};
use crate::state::player::Player;

pub trait Command {
    fn execute(&self, context: &mut Context, sink: &mut Sink);
}

pub struct MoveLeft;
impl Command for MoveLeft {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        let (obstacle_left, _id) = check_collision(context.all_maps[context.current_map_index].obstacles, &context.sprites, &context.player, true);

        if !obstacle_left {
            context.player.obstacle_left = false;

            // Update velocity if no collision is detected
            context.player.vx += ACCELERATION * 0.5;
            if context.player.vx > MAX_VELOCITY {
                context.player.vx = MAX_VELOCITY;
            } else {
                context.player.vx *= 0.98;
            }

            context.player.last_key = Some(Key::A);
            context.player.direction = Left;

            // Initialize a new field to track the frame count
            context.player.left_increment_frame_count += 1;

            if context.player.left_increment_frame_count >= 3 {
                context.player.left_increment_frame_count = 0; // Reset the frame count

                match context.player.left_increment {
                    7 => {
                        context.player.left_increment = 4;
                    }
                    _ => {
                        context.player.left_increment += 1;
                    }
                };
            }
        } else {
            // Stop the player from moving left if colliding
            context.player.vx = 0.0;
        }

        // Move player based on current velocity
        // context.player.x -= context.player.vx;
    }
}

pub struct MoveRight;

impl Command for MoveRight {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        let (obstacle_right, _id) = check_collision(context.all_maps[context.current_map_index].obstacles, &context.sprites, &context.player, false);

        if !obstacle_right {
            context.player.obstacle_right = false;

            // Update velocity if no collision is detected
            context.player.vx += ACCELERATION * 0.5;
            if context.player.vx > MAX_VELOCITY {
                context.player.vx = MAX_VELOCITY;
            } else {
                context.player.vx *= 0.98;
            }

            context.player.last_key = Some(Key::D);
            context.player.direction = Right;

            // Initialize a new field to track the frame count
            context.player.right_increment_frame_count += 1;

            if context.player.right_increment_frame_count >= 3 {
                context.player.right_increment_frame_count = 0; // Reset the frame count

                match context.player.right_increment {
                    3 => {
                        context.player.right_increment = 0;
                    }
                    _ => {
                        context.player.right_increment += 1;
                    }
                }
            }
        } else {
            // Stop the player from moving right if colliding
            context.player.vx = 0.0;
        }

        if context.footstep_active {
            if context.footstep_index == 4 { context.footstep_index = 0; }
            else { context.footstep_index += 1; }

            let file_path = match context.footstep_index {
                0 => "foot_step_1.wav",
                1 => "foot_step_2.wav",
                2 => "foot_step_3.wav",
                _ => "foot_step_4.wav",
            };

            let file = std::fs::File::open(file_path).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap().take_duration(std::time::Duration::from_millis(125));

            // Append the sound source to the audio sink for playback
            let _result = sink.append(source);


        }


    }
}

pub fn check_collision(obstacles: &Vec<Obstacle>, sprites: &Sprites, player: &Player, is_left: bool) -> (bool, Option<usize>) {
    let mut collision_id: Option<usize> = None;
    println!("----------------------------------------------------------------------");
    let collision = obstacles.iter().enumerate().any(|(index, obstacle)| {
        println!("Checking collision: id: {:?}, x_left: {}, x_right: {}, y_bottom: {}, y_top: {}", obstacle.id, obstacle.x_left, obstacle.x_right, obstacle.y_bottom, obstacle.y_top);

        if obstacle.active == false {
            println!("- - - - Obstacle is not active - - - -");
            return false;
        }

        let player_x = if is_left {
            player.x + (sprites.player[player.left_increment].width as f32 / 2.5)
        } else {
            player.x + (sprites.player[player.right_increment].width as f32 / 1.5)
        };

        if player_x > obstacle.x_left && player_x < obstacle.x_right {
            println!("Collision of x axis detected: player_x: {}, obstacle.x_left: {}, obstacle.x_right: {}", player_x, obstacle.x_left, obstacle.x_right);

            let magica = 25.0;
            if player.y >= obstacle.y_top + magica && player.y <= obstacle.y_bottom + magica {
                collision_id = Some(index);
                println!("Collision detected with obstacle id {:?} x.left {}, x.right: {}, obstacle.y_bottom: {}, obstacle.y_top: {}", obstacle.id, obstacle.x_left, obstacle.x_right , obstacle.y_bottom + magica, obstacle.y_top + magica);
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    if let Some(id) = collision_id {
        // println!("Collision detected at x: {}, y: {}, with obstacle id: {}", player.x, player.y, id);
    }

    (collision, collision_id)
}

pub struct Jump;

impl Command for Jump {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {

        if !context.player.is_jumping && (context.player.on_ground || context.player.on_obstacle) {
            context.player.vy = JUMP_VELOCITY;
            context.player.on_ground = false;
            context.player.on_obstacle = false;
            context.player.is_jumping = true;
            context.player.last_key = Some(Key::Space);

            let file = std::fs::File::open("jump.wav").unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap().take_duration(std::time::Duration::from_millis(1000));

            // Append the sound source to the audio sink for playback
            let _result = sink.append(source);
        }
    }
}

pub struct Kick;

impl Command for Kick {
    fn execute(&self, context: &mut Context, sink: &mut Sink) {
        context.player.is_kicking = true;
        context.player.kick_frame = 0;
        context.player.kick_frame_timer = 0;
        // TODO: Need to fix this as it currently fucks up everything

        // let sorted_obstacles = sort_obstacles_by_y(context.all_maps[context.current_map_index].obstacles);

        let (collision, id) = check_collision(context.all_maps[context.current_map_index].obstacles, &context.sprites, &context.player, context.player.direction == Left);

        // Check if the player is adjacent to an obstacle to the right
        if collision {

            let file = std::fs::File::open("kick_box.wav").unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap().take_duration(std::time::Duration::from_millis(1000));

            // Append the sound source to the audio sink for playback
            let _result = sink.append(source);

            // println!("Player is adjacent to an obstacle with id {} to the right.", id.unwrap());
            if context.all_maps[context.current_map_index].obstacles[id.unwrap()].durability > 0 {
                // println!("Obstacle durability: {}", context.all_maps[context.current_map_index].obstacles[id.unwrap()].durability);
                context.all_maps[context.current_map_index].obstacles[id.unwrap()].durability -= 1;
            } else {
                // println!("Obstacle durability: 0");
                remove_box(context, id.unwrap(), sink);
            }

        } else {
            let file = std::fs::File::open("kick.wav").unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap().take_duration(std::time::Duration::from_millis(1000));

            // Append the sound source to the audio sink for playback
            let _result = sink.append(source);
        }
    }
}

pub type CommandMap = HashMap<Key, Arc<dyn Command>>;

pub fn initialize_command_map() -> CommandMap {
    let mut commands: CommandMap = HashMap::new();

    commands.insert(Key::A, Arc::new(MoveLeft));
    commands.insert(Key::D, Arc::new(MoveRight));
    commands.insert(Key::Space, Arc::new(Jump));
    commands.insert(Key::X, Arc::new(Kick));

    commands
}

