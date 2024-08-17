use std::time::Duration;

use minifb::Key;

use crate::state::player::{Player, PlayerState};

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
    pub y_left: f32,
    pub y_right: f32
}

pub fn jump_obstacles(player: &mut Player, obstacles: &Vec<Obstacle>) {

    // Apply vertical velocity if jumping
    if player.is_jumping {
        player.y += player.vy;
    }

    // Check if player is almost on the ground
    if player.y >= 140.0 && player.y <= 160.0 {
        player.almost_ground = true;
        // player.state = PlayerState::AlmostGround;
    } else {
        player.almost_ground = false;
    }

    let mut on_any_obstacle = false;

    // Check for each obstacle
    for obstacle in obstacles {
        if player.x >= obstacle.x_left && player.x <= obstacle.x_right {
            if player.y >= obstacle.y_top && player.y <= obstacle.y_bottom {
                if player.state != PlayerState::OnObstacle {
                    // Player just landed on the obstacle
                    player.y = obstacle.y_bottom;
                    player.on_obstacle = true;
                    player.on_ground = false;
                    player.is_jumping = false;
                    player.state = PlayerState::OnObstacle;
                    player.vy = 0.0;
                    println!("Player just landed on obstacle {} at x: {}, y: {}", obstacle.id.0, player.x, player.y);
                } else {
                    // Player is already on the obstacle
                    player.on_obstacle = true;
                    player.on_ground = false;
                    println!("Player on obstacle {} at x: {}, y: {}", obstacle.id.0, player.x, player.y);
                }
                on_any_obstacle = true;
                break;
            } else if player.y < obstacle.y_top {
                // Player is above the obstacle but not touching it
                player.on_ground = false;
                player.on_obstacle = false;
                player.above_obstacle = true;
                player.state = PlayerState::InAir;
                player.is_jumping = true;
                println!("Player is in the air above obstacle {} at x: {}, y: {}", obstacle.id.0, player.x, player.y);
                on_any_obstacle = true;
                break;
            }
        }
    }

    if !on_any_obstacle {
        if player.y >= GROUND {
            // Player is on the ground (not on an obstacle)
            player.y = GROUND;
            player.vy = 0.0;
            player.on_ground = true;
            player.on_obstacle = false;
            player.is_jumping = false;
            player.state = PlayerState::OnGround;
            println!("Player is on the ground at x: {}, y: {}", player.x, player.y);
        } else {
            // Player is in the air (not above any obstacle)
            player.on_ground = false;
            player.on_obstacle = false;
            player.above_obstacle = false;
            player.state = PlayerState::InAir;
            player.is_jumping = true;
            println!("Player is in the air at x: {}, y: {}", player.x, player.y);
        }
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