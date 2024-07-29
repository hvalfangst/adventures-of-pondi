use std::time::Duration;

use minifb::Key;

pub mod event_loop;
pub mod utils;

const FRAME_DURATION: Duration = Duration::from_millis(16); // Approximately 60Hz refresh rate


pub struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    on_ground: bool,
    on_obstacle: bool,
    last_key: Option<Key>
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
            on_obstacle: true
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