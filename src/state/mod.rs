use std::time::Duration;

use minifb::Key;
use crate::state::player::Player;

pub mod event_loop;
pub mod update;
pub mod command;
pub mod global_command;
pub mod player;
mod input_handler;

const FRAME_DURATION: Duration = Duration::from_nanos(16666667); // 16.6666667 ms = 60 FPS
const BACKGROUND_CHANGE_INTERVAL: Duration = Duration::from_secs(1);

const GRAVITY: f32 = 0.5;
const JUMP_VELOCITY: f32 = -7.0;
const MAX_VELOCITY: f32 = 2.0;
const ACCELERATION: f32 = 0.5;
const FRICTION: f32 = 0.2;
const GROUND: f32 = 205.0;
const LOWER_BOUND: f32 = 0.0;
const UPPER_BOUND: f32 = 225.0;
const KICK_FRAME_DURATION: u32 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObstacleId(pub usize);

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub id: ObstacleId,
    pub x_left: f32,
    pub x_right: f32,
    pub y_top: f32,
    pub y_bottom: f32,
    pub y_transition_pos: f32,
}

pub fn jump_obstacles(mut player: &mut Player) {

    if player.x >= 70.0 && player.x <= 100.0 && player.y >= 175.0 && player.y <= 185.0 {
        player.y = 185.0;
        player.on_obstacle = true;
        player.on_ground = true;
        player.is_jumping = false;
    }

    else if player.x >= 113.0 && player.x <= 160.0 && player.y >= 175.0 && player.y <= 185.0 {

        player.y = 185.0;
        player.on_obstacle = true;
        player.on_ground = true;
        player.is_jumping = false;
    }

    else if player.x >= 160.0 && player.x <= 175.0 && player.y >= 175.0 && player.y <= 185.0 {

        player.y = 185.0;
        player.on_obstacle = true;
        player.on_ground = true;
        player.is_jumping = false;
    }


    else if player.x >= 174.0 && player.x <= 182.0 && player.y >= 160.0 && player.y <= 185.0 {

        player.y = 185.0;
        player.on_obstacle = true;
        player.on_ground = true;
        player.is_jumping = false;
    }

    else {
        player.on_ground = false;
        player.on_obstacle = false;
    }
}

fn apply_friction(player: &mut Player) {
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