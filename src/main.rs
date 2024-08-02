use crate::{
    graphics::sprites::Sprites,
    state::{event_loop::start_event_loop, Player}
};
use crate::graphics::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::state::Obstacle;

mod state;mod graphics;

fn main() {
    let sprites = Sprites::new();

    let mut player = Player::new(0.0 * TILE_WIDTH as f32, 11.0 * TILE_HEIGHT as f32);

    // Initialize obstacles
    let obstacles = vec![
        Obstacle { x: 7.0, y: 176.0, width: 16.0, height: 16.0 },
        Obstacle { x: 7.0, y: 160.0, width: 16.0, height: 16.0 }
    ];

    start_event_loop(&mut player, &sprites, &obstacles);
}