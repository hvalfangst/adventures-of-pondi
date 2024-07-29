use crate::graphics::constants::{TILE_HEIGHT, TILE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

pub mod sprites; pub mod constants;

pub fn get_amt_x_tiles() -> usize {
    return WINDOW_WIDTH / TILE_WIDTH;
}

pub fn get_amt_y_tiles() -> usize {
    return WINDOW_HEIGHT / TILE_HEIGHT;
}