use crate::{
    graphics::sprites::Sprites,
    state::{event_loop::start_event_loop, Player}
};
use crate::state::Obstacle;

mod state;mod graphics;

fn main() {
    let sprites = Sprites::new();

    let mut player = Player::new(1.0, 176.0);

    // Initialize obstacles
    let obstacles = vec![
        Obstacle { x: 70.0, y: 176.0, width: 16.0, height: 16.0 }
    ];

    start_event_loop(&mut player, &sprites, &obstacles, false);
}