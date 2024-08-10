use std::collections::HashSet;
use std::time::{Duration, Instant};
use minifb::Key;
use crate::state::ObstacleId;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub on_ground: bool,
    pub on_obstacle: bool,
    pub last_key: Option<Key>,
    pub left_increment: usize,
    pub right_increment: usize,
    pub direction: String,
    pub right_increment_frame_count: usize,
    pub left_increment_frame_count: usize,
    pub kick_frame: usize,
    pub kick_frame_timer: u32,
    kick_start_time: u32,
    pub is_kicking: bool,
    pub almost_ground: bool,
    pub obstacle_left: bool,
    pub obstacle_right: bool,
    pub last_jump_time: Instant,
    pub jump_cooldown: Duration,
    pub on_obstacles: HashSet<ObstacleId>
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
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
            on_obstacles: HashSet::new()
        }
    }
}