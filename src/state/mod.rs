use std::time::Duration;

use minifb::Key;

pub mod event_loop;
pub mod utils;

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


pub struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    on_ground: bool,
    on_obstacle: bool,
    last_key: Option<Key>,
    left_increment: usize,
    right_increment: usize,
    direction: String,
    right_increment_frame_count: usize,
    left_increment_frame_count: usize,
    kick_frame: usize,
    kick_frame_timer: u32,
    kick_start_time: u32,
    is_kicking: bool,
    almost_ground: bool
}

impl Player {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
            last_key: None,
            on_obstacle: true,
            left_increment: 4,
            right_increment: 0,
            direction: "RIGHT".parse().unwrap(),
            right_increment_frame_count: 0,
            left_increment_frame_count: 0,
            kick_frame: 0,
            kick_frame_timer: 0,
            kick_start_time: 0,
            is_kicking: true,
            almost_ground: false
        }
    }
}

pub struct Obstacle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

pub enum GameState {
    Empty,
    Obstacle,
}