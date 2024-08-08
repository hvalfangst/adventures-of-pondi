use std::alloc::System;
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
    if window.is_key_pressed(Key::Space, KeyRepeat::Yes) && player.on_ground {
        player.vy = JUMP_VELOCITY;
        player.on_ground = false;
        player.last_key = Some(Key::Space);
    }

    // Handle movement to the right
    if window.is_key_pressed(Key::D, KeyRepeat::Yes) {


        // Calculate future position based on current position, velocity, and acceleration
        let future_position = player.x + (player.vx + ACCELERATION);

        // TODO: implement
        // // Check if future position will collide with any obstacle
        // let collision: bool = obstacles.iter().any(|obs| {
        //     // Check if the future position overlaps with any obstacle
        //     future_position > obs.x && player.x < obs.x
        // });

        if !(player.x > 70.0 && player.x < 96.0 && player.on_ground) {
            // Update velocity if no collision is detected
            player.vx += ACCELERATION;
            if player.vx > MAX_VELOCITY {
                player.vx = MAX_VELOCITY;
            }

            player.last_key = Some(Key::D);
            player.direction = "RIGHT".parse().unwrap();

            // Initialize a new field to track the frame count
            player.right_increment_frame_count += 1;

            if player.right_increment_frame_count >= 3 {
                player.right_increment_frame_count = 0; // Reset the frame count

                match player.right_increment {
                    3 => {
                        player.right_increment = 0;
                    }
                    _ => {
                        player.right_increment += 1;
                    }
                }
            }

            // Move player based on current velocity
            player.x += player.vx;

        }   else {
        // Handle collision response
        // Stop the player from moving right if colliding
        player.vx = 0.0;
        // Optionally, adjust position to resolve overlap (if necessary)
    }

        // Handle movement to the left
    } else if window.is_key_pressed(Key::A, KeyRepeat::Yes) {

        // TODO: implement
        // // Calculate future position based on current position, velocity, and acceleration
        // let future_position = player.x + (player.vx + ACCELERATION);
        //
        // // Check if future position will collide with any obstacle
        // let collision: bool = obstacles.iter().any(|obs| {
        //     // Check if the future position overlaps with any obstacle
        //     // future_position < obs.x
        //     false
        // });

        if !(player.x < 98.0 && player.x > 70.0 && player.on_ground)  {
            // Update velocity if no collision is detected
            player.vx += ACCELERATION;
            if player.vx > MAX_VELOCITY {
                player.vx = MAX_VELOCITY;
            }

            player.last_key = Some(Key::A);
            player.direction = "LEFT".parse().unwrap();

            // Initialize a new field to track the frame count
            player.left_increment_frame_count += 1;

            if player.left_increment_frame_count >= 3 {
                player.left_increment_frame_count = 0; // Reset the frame count


                match player.left_increment {
                    7 => {
                        player.left_increment = 4;
                    }
                    _ => {
                        player.left_increment += 1;
                    }
                };
            }

            // Move player based on current velocity
            player.x -= player.vx;

        }  else {
            // Handle collision response
            // Stop the player from moving right if colliding
            player.vx = 0.0;
            // Optionally, adjust position to resolve overlap (if necessary)
        }
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

    // player.x += player.vx;
    player.y += player.vy;

    // Reset vertical velocity and flag if player is on the ground
    if player.y >= GROUND {
        player.on_ground = true;
        player.vy = 0.0;
        player.y = GROUND;
    } else {
        player.on_ground = false;
    }

    // Prevent the player from moving out of bounds
    if player.x < LOWER_BOUND {
        player.x = LOWER_BOUND;
        player.vx = 0.0;
    } else if player.x > UPPER_BOUND {
        player.x = UPPER_BOUND;
        player.vx = 0.0;
    }

    println!("Player X: {}, Player Y: {}\n", player.x, player.y)
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


pub fn update_buffer_with_state(player: &mut Player, sprites: &Sprites, window_buffer: &mut Vec<u32>, background_index: usize) {
    draw_background_sprite(sprites, window_buffer, background_index);
    draw_player(sprites, window_buffer, player)
}

pub fn draw_player(sprites: &Sprites, window_buffer: &mut Vec<u32>, player: &mut Player) {
    // Determine the current direction and action of the player
    let direction = player.direction.as_str();
    let last_key = player.last_key.unwrap_or(Key::D);

    // Print the last key for debugging
    println!("Last key pressed: {:?}", last_key);

    // Determine the sprite to draw
    let sprite_to_draw = if last_key == Key::X {

        if !player.is_kicking {
            // Start the kick animation if not already kicking
            player.is_kicking = true;
            player.kick_frame = 0; // Start with the first kick frame
            player.kick_frame_timer = 0; // Reset the timer
        }

        // Update kick animation
        if player.is_kicking {
            player.kick_frame_timer += 1;
            if player.kick_frame_timer >= KICK_FRAME_DURATION {
                player.kick_frame += 1; // Move to the next frame
                player.kick_frame_timer = 0;

                // Check if animation has completed
                if player.kick_frame >= 2 { // Two frames have been played
                    player.is_kicking = false; // Stop kicking animation
                    player.kick_frame = 0; // Reset frame to 0
                }
            }
        }

        // Select the correct kick frame based on direction
        if player.is_kicking  && direction == "RIGHT" {
            &sprites.kick[player.kick_frame]
        } else if player.is_kicking && direction == "LEFT" {
            &sprites.kick[2 + player.kick_frame] // Assuming LEFT frames are 2 and 3
        } else {
            // Default to the player sprite if not kicking
            &sprites.player[player.right_increment]
        }


    }   else if !player.on_ground && direction == "RIGHT" {
        &sprites.jump[1]
    } else if !player.on_ground && direction == "LEFT" {
        &sprites.jump[4]
    } else if direction == "RIGHT" {
        &sprites.player[player.right_increment]
    } else if direction == "LEFT" {
        &sprites.player[player.left_increment]
    } else {
        &sprites.player[player.right_increment]
    };

    // Draw the chosen player sprite
    draw_sprite(
        player.x as usize,
        player.y as usize - (sprite_to_draw.height - 5) as usize,
        sprite_to_draw,
        window_buffer,
        WINDOW_WIDTH
    );

    let shadow_sprite = match player.on_ground {
        true => &sprites.shadow[0],
        _ => &sprites.shadow[1]
    };

    // Draw associated shadow
    draw_sprite(
        player.x as usize,
        GROUND as usize + 3,
        shadow_sprite,
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