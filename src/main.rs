use crate::{
    graphics::sprites::Sprites,
    state::{event_loop::start_event_loop}
};
use crate::state::{Obstacle, ObstacleId};
use crate::state::command::{initialize_command_map};
use crate::state::global_command::initialize_global_command_map;
use crate::state::player::Player;

mod state;mod graphics;



fn main() {
    let sprites = Sprites::new();

    let obstacles = vec![
        Obstacle { id: ObstacleId(1),  x_left: 70.5, x_right: 98.0, y_bottom: 185.0, y_top: 175.0, y_left: 205.0, y_right: 205.0},
        Obstacle { id: ObstacleId(2), x_left: 107.0, x_right: 160.0, y_bottom: 185.0, y_top: 175.0, y_left: 205.0, y_right: 205.0},
        Obstacle { id: ObstacleId(3), x_left: 172.0, x_right: 195.0, y_bottom: 170.0, y_top: 162.0, y_left: 185.0, y_right: 185.0},
        Obstacle { id: ObstacleId(4), x_left: 191.0, x_right: 215.0, y_bottom: 160.0, y_top: 150.0, y_left: 205.0, y_right: 170.0},
     ];

    let mut player = Player::new(1.0, 176.0);

    let commands = initialize_command_map();
    let global_commands = initialize_global_command_map();

    start_event_loop(&mut player, &sprites, &obstacles, false, commands, global_commands);
}