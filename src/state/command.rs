use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::sync::Arc;

use crate::graphics::sprites::Sprites;
use crate::state::player::Player;
use crate::state::Direction::{Left, Right};
use crate::state::{remove_box, GameState, Obstacle, ACCELERATION, JUMP_SOUND, JUMP_VELOCITY, KICK_BOX_SOUND, KICK_SOUND, MAX_VELOCITY, WALK_SOUND_1, WALK_SOUND_2, WALK_SOUND_3, WALK_SOUND_4};
use minifb::Key;
use rodio::{Sink, Source};

pub trait Command {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub struct MoveLeft;
impl Command for MoveLeft {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let (obstacle_left, _id) = check_collision(game_state.all_maps[game_state.current_map_index].obstacles, &game_state.sprites, &game_state.player, true);

        if !obstacle_left {
            game_state.player.obstacle_left = false;

            // Update velocity if no collision is detected
            game_state.player.vx += ACCELERATION * 0.5;
            if game_state.player.vx > MAX_VELOCITY {
                game_state.player.vx = MAX_VELOCITY;
            } else {
                game_state.player.vx *= 0.98;
            }

            game_state.player.last_key = Some(Key::A);
            game_state.player.direction = Left;

            // Initialize a new field to track the frame count
            game_state.player.left_increment_frame_count += 1;

            if game_state.player.left_increment_frame_count >= 3 {
                game_state.player.left_increment_frame_count = 0; // Reset the frame count

                match game_state.player.left_increment {
                    7 => {
                        game_state.player.left_increment = 4;
                    }
                    _ => {
                        game_state.player.left_increment += 1;
                    }
                };
            }
        } else {
            // Stop the player from moving left if colliding
            game_state.player.vx = 0.0;
        }

        // Move player based on current velocity
        // game_state.player.x -= game_state.player.vx;
    }
}

pub struct MoveRight;

impl Command for MoveRight {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let (obstacle_right, _id) = check_collision(game_state.all_maps[game_state.current_map_index].obstacles, &game_state.sprites, &game_state.player, false);

        if !obstacle_right {
            game_state.player.obstacle_right = false;

            // Update velocity if no collision is detected
            game_state.player.vx += ACCELERATION * 0.5;
            if game_state.player.vx > MAX_VELOCITY {
                game_state.player.vx = MAX_VELOCITY;
            } else {
                game_state.player.vx *= 0.98;
            }

            game_state.player.last_key = Some(Key::D);
            game_state.player.direction = Right;

            // Initialize a new field to track the frame count
            game_state.player.right_increment_frame_count += 1;

            if game_state.player.right_increment_frame_count >= 3 {
                game_state.player.right_increment_frame_count = 0; // Reset the frame count

                match game_state.player.right_increment {
                    3 => {
                        game_state.player.right_increment = 0;
                    }
                    _ => {
                        game_state.player.right_increment += 1;
                    }
                }
            }
        } else {
            // Stop the player from moving right if colliding
            game_state.player.vx = 0.0;
        }

        if game_state.footstep_active {
            if game_state.footstep_index == 4 { game_state.footstep_index = 0; }
            else { game_state.footstep_index += 1; }

            let sound_index = match game_state.footstep_index {
                0 => WALK_SOUND_1,
                1 => WALK_SOUND_2,
                2 => WALK_SOUND_3,
                _ => WALK_SOUND_4,
            };

            // let file = &game_state.sounds[sound_index];;
            // let source = rodio::Decoder::new(BufReader::new(file)).unwrap().take_duration(std::time::Duration::from_millis(125));
            // let _result = sink.append(source);
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
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        if !game_state.player.is_jumping && (game_state.player.on_ground || game_state.player.on_obstacle) {
            game_state.player.vy = JUMP_VELOCITY;
            game_state.player.on_ground = false;
            game_state.player.on_obstacle = false;
            game_state.player.is_jumping = true;
            game_state.player.last_key = Some(Key::Space);


            let file = &game_state.sounds[JUMP_SOUND]; // Get the raw sound data (Vec<u8>)
            let cursor = Cursor::new(file.clone()); // Clone to create an owned Cursor<Vec<u8>>

            let source = rodio::Decoder::new(BufReader::new(cursor))
                .unwrap()
                .take_duration(std::time::Duration::from_millis(1000));

            sink.append(source); // Play the sound

        }
    }
}

pub struct Kick;

impl Command for Kick {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        game_state.player.is_kicking = true;
        game_state.player.kick_frame = 0;
        game_state.player.kick_frame_timer = 0;
        // TODO: Need to fix this as it currently fucks up everything

        // let sorted_obstacles = sort_obstacles_by_y(game_state.all_maps[game_state.current_map_index].obstacles);

        let (collision, id) = check_collision(game_state.all_maps[game_state.current_map_index].obstacles, &game_state.sprites, &game_state.player, game_state.player.direction == Left);

        // Check if the player is adjacent to an obstacle to the right
        if collision {

            let file = &game_state.sounds[KICK_BOX_SOUND]; // Get the raw sound data (Vec<u8>)
            let cursor = Cursor::new(file.clone()); // Clone to create an owned Cursor<Vec<u8>>

            let source = rodio::Decoder::new(BufReader::new(cursor))
                .unwrap()
                .take_duration(std::time::Duration::from_millis(1000));

            sink.append(source); // Play the sound

            // println!("Player is adjacent to an obstacle with id {} to the right.", id.unwrap());
            if game_state.all_maps[game_state.current_map_index].obstacles[id.unwrap()].durability > 0 {
                // println!("Obstacle durability: {}", game_state.all_maps[game_state.current_map_index].obstacles[id.unwrap()].durability);
                game_state.all_maps[game_state.current_map_index].obstacles[id.unwrap()].durability -= 1;
            } else {
                // println!("Obstacle durability: 0");
                remove_box(game_state, id.unwrap(), sink);
            }

        } else {
            let file = &game_state.sounds[KICK_SOUND]; // Get the raw sound data (Vec<u8>)
            let cursor = Cursor::new(file.clone()); // Clone to create an owned Cursor<Vec<u8>>

            let source = rodio::Decoder::new(BufReader::new(cursor))
                .unwrap()
                .take_duration(std::time::Duration::from_millis(1000));

            sink.append(source); // Play the sound
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

