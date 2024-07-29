use crate::{
    state::{event_loop::start_event_loop, Player},
    graphics::sprites::Sprites
};
use crate::state::Obstacle;
use crate::state::utils::{create_state_array, read_map_file};

mod state;mod graphics;

fn main() {
    let sprites = Sprites::new();


    let filename = "map_1.txt";
    let map_lines = read_map_file(filename).unwrap();
    let tile_width = sprites.tile[0].width as f32;
    let tile_height = sprites.tile[0].height as f32;
    let mut state_array = create_state_array(&map_lines, tile_width as usize, tile_height as usize);
    let mut player = Player::new(0.0 * tile_width, 11.0 * tile_height);


    // Initialize obstacles
    let obstacles = vec![
        Obstacle { x: 7.0, y: 176.0, width: 16.0, height: 16.0 },
        Obstacle { x: 7.0, y: 160.0, width: 16.0, height: 16.0 }

    ];

    start_event_loop(&mut player, &sprites, &mut state_array, &obstacles);
}