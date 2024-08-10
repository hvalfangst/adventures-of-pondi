use std::time::Duration;

use minifb::Key;

pub mod event_loop;
pub mod update;
pub mod command;
pub mod global_command;
pub mod player;
mod input_handler;

const FRAME_DURATION: Duration = Duration::from_millis(16); // Approximately 60Hz refresh rate
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
    pub id: ObstacleId, // Add an ID to identify each obstacle
    pub x_left: f32,
    pub x_right: f32,
    pub y_top: f32,
    pub y_bottom: f32,
    pub y_transition_pos: f32,
}