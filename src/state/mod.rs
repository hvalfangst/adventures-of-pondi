use std::io::BufReader;
use std::time::Duration;

use minifb::{Key, Window};
use rodio::Source;
use crate::graphics::sprites::Sprites;
use crate::state::player::{Player, PlayerState};
use crate::{sort_obstacles_by_y, Tile};

pub mod event_loop;
pub mod update;
pub mod command;
pub mod global_command;
pub mod player;
mod input_handler;

const FRAME_DURATION: Duration = Duration::from_nanos(16666667); // 16.6666667 ms = 60 FPS
const BACKGROUND_CHANGE_INTERVAL: Duration = Duration::from_secs(1);

const GRAVITY: f32 = 0.5;
const JUMP_VELOCITY: f32 = -5.0;
const MAX_VELOCITY: f32 = 2.0;
const ACCELERATION: f32 = 0.5;
const FRICTION: f32 = 0.2;
const GROUND: f32 = 205.0;
const LOWER_BOUND: f32 = 0.0;
const UPPER_BOUND: f32 = 225.0;
const KICK_FRAME_DURATION: u32 = 8;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObstacleId(pub usize);

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub id: ObstacleId,
    pub x_left: f32,
    pub x_right: f32,
    pub y_top: f32,
    pub y_bottom: f32,
    pub velocity_y: f32, // For gravity
    pub falling: bool,   // Whether it's falling
    pub active: bool,    // If false, box is removed
    pub durability: u8,  // Health of the box
}
pub fn jump_obstacles(mut context: &mut Context, sink: &mut rodio::Sink) {

    // Apply vertical velocity if jumping
    if context.player.is_jumping {
        context.player.y += context.player.vy;
    }

    // Check if context.player is almost on the ground
    if context.player.y >= 140.0 && context.player.y <= 160.0 {
        context.player.almost_ground = true;
    } else {
        context.player.almost_ground = false;
    }

    let mut on_any_obstacle = false;

    // Check for each obstacle
    for obstacle in context.all_maps[context.current_map_index].obstacles.iter() {

        if obstacle.active == false {
            continue;
        }

        if context.player.x + 10.0 > obstacle.x_left && context.player.x + 5.0 < obstacle.x_right {
            if context.player.y <= obstacle.y_bottom && context.player.y >= obstacle.y_top {
                 // println!("context.player.y: {}, obstacle.y_bottom: {}, obstacle.y_top: {}", context.player.y, obstacle.y_bottom, obstacle.y_top);
                if context.player.state != PlayerState::OnObstacle {
                    // player just landed on the obstacle
                    context.player.y = obstacle.y_bottom - 1.0;
                    context.player.on_obstacle = true;
                    context.player.on_ground = false;
                    context.player.is_jumping = false;
                    context.player.state = PlayerState::OnObstacle;
                    context.player.vy = 0.0;
                } else {
                    // context.player is already on the obstacle
                    context.player.on_obstacle = true;
                    context.player.on_ground = false;
                }
                on_any_obstacle = true;
                break;
            } else if context.player.y < obstacle.y_top {
                // player is above the obstacle but not touching it
                context.player.on_ground = false;
                context.player.on_obstacle = false;
                context.player.above_obstacle = true;
                context.player.state = PlayerState::InAir;
                context.player.is_jumping = true;
                on_any_obstacle = true;
                break;
            }
        }
    }

    if !on_any_obstacle {
        if context.player.y >= GROUND {
            // player is on the ground (not on an obstacle)
            context.player.y = GROUND;
            context.player.vy = 0.0;
            context.player.on_ground = true;
            context.player.on_obstacle = false;
            context.player.is_jumping = false;

            if context.player.state == PlayerState::InAir {
                let file = std::fs::File::open("fall_mild.wav").unwrap();
                let source = rodio::Decoder::new(BufReader::new(file)).unwrap().take_duration(std::time::Duration::from_millis(1000));

                // Append the sound source to the audio sink for playback
                let _result = sink.append(source);
            }

            context.player.state = PlayerState::OnGround;


        } else {
            // player is in the air (not above any obstacle)
            context.player.on_ground = false;
            context.player.on_obstacle = false;
            context.player.above_obstacle = false;
            context.player.state = PlayerState::InAir;
            context.player.is_jumping = true;
        }
    }
}


fn apply_friction(mut context: &mut Context) {
    if context.player.vx > 0.0 {
        context.player.vx -= FRICTION;
        if context.player.vx < 0.0 {
            context.player.vx = 0.0;
        }
    } else if context.player.vx < 0.0 {
        context.player.vx += FRICTION;
        if context.player.vx > 0.0 {
            context.player.vx = 0.0;
        }
    }
}

pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }

    pub fn update(&mut self, player_x: f32, player_y: f32) {
        self.x = player_x - self.width / 2.0;
        self.y = player_y - self.height / 2.0;
    }
}

fn remove_box(context: &mut Context, box_index: usize, sink: &mut rodio::Sink) {
    println!("Removing box {}", box_index);
    let mut to_remove = false;
    if context.all_maps[context.current_map_index].obstacles[box_index].active {
        println!("Box is active");
        // Obtain the x_left and x_right values of the removed box
        let removed_box_x_left = context.all_maps[context.current_map_index].obstacles[box_index].x_left;
        let removed_box_x_right = context.all_maps[context.current_map_index].obstacles[box_index].x_right;
        let removed_box_y_top = context.all_maps[context.current_map_index].obstacles[box_index].y_top;

        println!("Box x_left: {}, x_right: {}", removed_box_x_left, removed_box_x_right);

        // Remove the box

        println!("Box {} removed", box_index);

        let file = std::fs::File::open("explosion.wav").unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap().take_duration(std::time::Duration::from_millis(400));

        // Append the sound source to the audio sink for playback
        let _result = sink.append(source);

        // Shift all boxes above the removed box down by 16 pixels
        for i in 0..context.all_maps[context.current_map_index].obstacles.len() {
            println!("Box id: {}", i);

            let obstacle = &mut context.all_maps[context.current_map_index].obstacles[i];
            println!("Box {} x_left: {}, x_right: {}", i, obstacle.x_left, obstacle.x_right);
            if obstacle.x_left >= removed_box_x_left && obstacle.x_right <= removed_box_x_right { //&& obstacle.y_top < removed_box_y_top {
                obstacle.falling = true;
                obstacle.velocity_y = 0.0;
                println!("Box {} is falling", i);
                // Remove the box
                to_remove = true;
            } else {
                println!("Box {} is not falling. obs.x_left: {} removed.x_left: {} obs.x_right {} removed.x_right {}", i, obstacle.x_left, removed_box_x_left, obstacle.x_right, removed_box_x_right);
            }
        }
    }
    if to_remove {
        context.all_maps[context.current_map_index].obstacles.remove(box_index);
        println!("Box {} removed", box_index);
    }
}


pub struct Map<'a> {
    pub id: usize,
    pub tiles: Vec<Tile>,
    pub obstacles: &'a mut Vec<Obstacle>,
    pub width: usize,
    pub height: usize,
    pub starting_x: f32,
    pub starting_y: f32,
    pub transition_x: f32,
    pub transition_y: f32
}

pub struct Context<'a> {
    pub player: Player,
    pub sprites: Sprites,
    pub window_buffer: &'a mut Vec<u32>,
    pub grass_sprite_index: usize,
    pub sky_sprite_index: usize,
    pub window_width: usize,
    pub window_height: usize,
    pub window: &'a mut Window,
    pub scaled_buffer: &'a mut Vec<u32>,
    pub game_over_index: usize,
    pub viewport: Viewport,
    pub all_maps: Vec<Map<'a>>,
    pub current_map_index: usize,
    pub footstep_index: usize,
    pub footstep_active: bool
}