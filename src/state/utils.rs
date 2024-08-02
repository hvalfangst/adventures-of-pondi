use minifb::{Key, KeyRepeat, Window};

use crate::graphics::constants::*;
use crate::graphics::sprites::{draw_sprite, Sprites};
use crate::state::{*, Obstacle, Player};

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
        player.vy += GRAVITY;
    }

    // Handle jump iff player is on the ground
    if window.is_key_pressed(Key::Space, KeyRepeat::No) && player.on_ground {
        player.vy = JUMP_VELOCITY;
        player.on_ground = false;
    }

    // Handle movement to the right
    if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
        player.vx += ACCELERATION;
        if player.vx > MAX_VELOCITY {
            player.vx = MAX_VELOCITY;
        }

        player.last_key = Some(Key::D);
        player.direction = "RIGHT".parse().unwrap();

        match player.right_increment {
            0 => {
                player.right_increment = 1;
            }
            _ => {
                player.right_increment = 0;
            }
        };

        // Handle movement to the left
    } else if window.is_key_pressed(Key::A, KeyRepeat::Yes) {
        player.vx -= ACCELERATION;
        if player.vx < -MAX_VELOCITY {
            player.vx = -MAX_VELOCITY;
        }

        player.last_key = Some(Key::A);
        player.direction = "LEFT".parse().unwrap();

        match player.left_increment {
            3 => {
                player.left_increment = 4;
            }
            _ => {
                player.left_increment = 3;
            }
        };

    } else if window.is_key_pressed(Key::X, KeyRepeat::Yes) {
        player.last_key = Some(Key::X);

        match player.direction.as_str() {
            "RIGHT" => player.right_increment = 2,
            "LEFT"  => player.left_increment = 5,
            _ => {}
        };

    } else {
        // Apply friction to gradually slow down the player
        if player.vx > 0.0 {
            player.vx -= FRICTION;
            if player.vx < 0.0 {
                player.vx = 0.0;
            }
        } else if player.vx < 0.0 {
            player.vx += FRICTION;
            if player.vx > 0.0 {
                player.vx = 0.0;
            }
        }
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
            player.y = obstacle.y - 1.0;
            break;
        }
    }

    // Reset vertical velocity and flag if player is on the ground
    if player.y >= GROUND {
        player.on_ground = true;
        player.vy = 0.0;
        player.y = GROUND;
    } else {
        player.on_ground = false;
    }

    // Prevent the player from moving out of bounds and handle obstacle collisions
    if player.x < LOWER_BOUND {
        player.x = 0.0;
        player.vx = 0.0;
    } else if player.x > UPPER_BOUND {
        player.x = UPPER_BOUND;
        player.vx = 0.0;
    } else if is_colliding_with_obstacle(player, obstacles, player.vx, 0.0) {
        player.x -= player.vx;
        player.vx = 0.0;
    }
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


pub fn update_buffer_with_state(player: &Player, sprites: &Sprites, window_buffer: &mut Vec<u32>, background_index: usize) {
    draw_background_sprite(sprites, window_buffer, background_index);
    draw_player(sprites, window_buffer, player)
}

pub fn draw_player(sprites: &Sprites, window_buffer: &mut Vec<u32>, player: &Player) {

    let frame_index = match player.last_key.unwrap_or(Key::D) {
        Key::D | Key::X if player.direction.as_str() == "RIGHT" => player.right_increment,
        Key::A | Key::X if player.direction.as_str() == "LEFT" => player.left_increment,
        _ => player.right_increment,
    };

    draw_sprite(
        player.x as usize,
        player.y as usize,
        &sprites.player[frame_index],
        window_buffer,
        WINDOW_WIDTH
    );
}

pub struct GameState {
    character: char,
    x_range: (usize, usize),
    y_range: (usize, usize),
}

fn draw_background_sprite(sprites: &Sprites, buffer: &mut [u32], background_index: usize) {
    draw_sprite(0, 0, &sprites.background[background_index], buffer, WINDOW_WIDTH);
}

/// Draws the current window with the provided pixel buffer.
///
/// # Parameters
/// - `window`: Mutable reference to the `Window` object where the visuals are displayed.
/// - `window_buffer`: Mutable reference to a vector of `u32` representing the pixel data to be displayed.
pub fn draw_buffer(window: &mut Window, window_buffer: &mut Vec<u32>) {
    window.update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
}