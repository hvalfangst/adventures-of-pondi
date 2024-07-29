use std::fs::File;
use std::io;
use std::io::BufRead;

use minifb::{Key, KeyRepeat, Window};

use crate::graphics::constants::*;
use crate::graphics::sprites::{draw_sprite, Sprites};
use crate::state::{Obstacle, Player};

fn safe_get_char(game_state: &Vec<Vec<GameState>>, row: isize, col: isize) -> Option<char> {
    if row >= 0 && row < game_state.len() as isize {
        if col >= 0 && col < game_state[row as usize].len() as isize {
            return Some(game_state[row as usize][col as usize].character);
        }
    }
    None
}

pub fn handle_key_presses(player: &mut Player, window: &mut Window, obstacles: &Vec<Obstacle>) {
    // Apply gravity if player is not on ground
    if !player.on_ground {
        player.vy += 0.5; // Gravity constant
    }

    // Update position based on velocity
    player.x += player.vx;
    player.y += player.vy;

    // Check for collisions with obstacles
    player.on_obstacle = false;
    for obstacle in obstacles {
        if player.y <= obstacle.y + obstacle.height &&
            player.y + player.vy >= obstacle.y &&
            player.x + player.vx >= obstacle.x &&
            player.x <= obstacle.x + obstacle.width {
            player.on_obstacle = true;
            player.vy = 0.0;
            player.y = obstacle.y - 1.0; // Position player just above the obstacle
            break;
        }
    }

    // Check if player is on the ground
    if player.y >= 176.0 {
        player.on_ground = true;
        player.vy = 0.0;
        player.y = 176.0;
    } else {
        player.on_ground = false;
    }

    // Handle jump
    if window.is_key_pressed(Key::Space, KeyRepeat::No) && player.on_ground {
        player.vy = -10.0; // Initial jump velocity
        player.on_ground = false;
    }


    // Handle horizontal movement
    if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
        if player.x <= 235.0 && !is_colliding_with_obstacle(player, obstacles, 1.0, 0.0) {
            player.x += 1.0;
            player.last_key = Some(Key::D);
        }
    }

    if window.is_key_pressed(Key::A, KeyRepeat::Yes) {
        if player.x >= 0.0 && !is_colliding_with_obstacle(player, obstacles, -1.0, 0.0) {
            player.x -= 1.0;
            player.last_key = Some(Key::A);
        } else {
            println!("Out of bounds: {}", player.x);
        }
    }

    // Print player position for debugging
    println!("Player Position - x: {}, y: {}", player.x, player.y);
}

fn is_colliding_with_obstacle(player: &Player, obstacles: &Vec<Obstacle>, dx: f32, dy: f32) -> bool {
    for obstacle in obstacles {
        if player.x + dx < obstacle.x + obstacle.width &&
            player.x + dx + player.vx > obstacle.x &&
            player.y + dy < obstacle.y + obstacle.height &&
            player.y + dy + player.vy > obstacle.y {
            return true;
        }
    }
    false
}


pub fn update_buffer_with_state(player: &Player, sprites: &Sprites, window_buffer: &mut Vec<u32>, game_state: &Vec<Vec<GameState>>) {
    draw_background_sprite(sprites, window_buffer, game_state);

    draw_woodworm(sprites, window_buffer, player)
}

pub fn draw_woodworm(sprites: &Sprites, window_buffer: &mut Vec<u32>, player: &Player) {

    let frame_index = match player.last_key.unwrap_or(Key::D) {
        Key::D => 0,
        Key::A => 1,
        _ => 0,
    };

    draw_sprite(
        player.x as usize,
        player.y as usize,
        &sprites.kokemakken[frame_index],
        window_buffer,
        WINDOW_WIDTH
    );
}

pub fn read_map_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

pub struct GameState {
    character: char,
    x_range: (usize, usize),
    y_range: (usize, usize),
}

pub fn create_state_array(map_lines: &[String], tile_width: usize, tile_height: usize) -> Vec<Vec<GameState>> {
    let mut state_array = Vec::new();

    for (i, line) in map_lines.iter().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.char_indices() {
            let state = GameState {
                character: c,
                x_range: (j * tile_width, (j + 1) * tile_width),
                y_range: (i * tile_height, (i + 1) * tile_height),
            };
            row.push(state);
        }
        state_array.push(row);
    }

    state_array
}

fn draw_background_sprite(sprites: &Sprites, buffer: &mut [u32], game_state: &Vec<Vec<GameState>>) {
    for (row_idx, row) in game_state.iter().enumerate() {
        for (col_idx, state) in row.iter().enumerate() {
            match state.character {
                'X' => {
                    // println!("Drawing obstacle sprite at X Range: {:?}, Y Range: {:?}", state.x_range, state.y_range);
                    draw_sprite(state.x_range.0, state.y_range.0, &sprites.tile[0], buffer, WINDOW_WIDTH);
                },
                'O' => {
                    // println!("Drawing empty sprite at X Range: {:?}, Y Range: {:?}", state.x_range, state.y_range);
                    draw_sprite(state.x_range.0, state.y_range.0, &sprites.tile[1], buffer, WINDOW_WIDTH);
                },
                _ => {
                    // println!("Unknown character '{}' at position row {}, col {}", state.character, row_idx, col_idx);
                }
            }
        }
    }
}

/// Draws the current window with the provided pixel buffer.
///
/// # Parameters
/// - `window`: Mutable reference to the `Window` object where the visuals are displayed.
/// - `window_buffer`: Mutable reference to a vector of `u32` representing the pixel data to be displayed.
pub fn draw_buffer(window: &mut Window, window_buffer: &mut Vec<u32>) {
    window.update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
}