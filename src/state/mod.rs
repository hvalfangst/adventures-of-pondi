use std::time::{Duration, Instant};

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
    almost_ground: bool,
    obstacle_left: bool,
    obstacle_right: bool,
    last_jump_time: Instant,
    jump_cooldown: Duration, // e.g., Duration::new(1, 0) for a 1-second cooldown
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
            on_obstacle: false,
            left_increment: 4,
            right_increment: 0,
            direction: "RIGHT".parse().unwrap(),
            right_increment_frame_count: 0,
            left_increment_frame_count: 0,
            kick_frame: 0,
            kick_frame_timer: 0,
            kick_start_time: 0,
            is_kicking: true,
            almost_ground: false,
            obstacle_left: false,
            obstacle_right: false,
            last_jump_time: Instant::now(),
            jump_cooldown: Duration::new(1, 0), // 1-second cooldown
        }
    }
}

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
    pub y_position: f32,
}

pub enum GameState {
    Empty,
    Obstacle,
}