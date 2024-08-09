use crate::{
    graphics::sprites::Sprites,
    state::{event_loop::start_event_loop, Player}
};
use crate::state::Obstacle;

mod state;mod graphics;

fn main() {
    let sprites = Sprites::new();

    let mut player = Player::new(1.0, 176.0);


    let obstacles = vec![
         Obstacle { x_range: (70.5, 98.0), y_range: (160.0, 180.0), y_position: 187.0},
         // Obstacle { x_range: (104.0, 158.0), y_range: (160.0, 180.0), y_position: 187.0}
    ];

    start_event_loop(&mut player, &sprites, obstacles, false);
}