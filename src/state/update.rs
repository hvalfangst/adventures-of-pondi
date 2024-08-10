use minifb::{Key, Window};

use crate::graphics::constants::*;
use crate::graphics::sprites::{draw_sprite, Sprites};
use crate::state::*;
use crate::state::player::Player;

pub fn update(player: &mut Player, sprites: &Sprites, window_buffer: &mut Vec<u32>, background_index: usize) {
    draw_background_sprite(sprites, window_buffer, background_index);
    draw_player(sprites, window_buffer, player)
}

fn draw_player(sprites: &Sprites, window_buffer: &mut Vec<u32>, player: &mut Player) {
    // Determine the current direction and action of the player
    let direction = player.direction.as_str();
    let last_key = player.last_key.unwrap_or(Key::D);

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

    }   else if player.almost_ground && !player.on_obstacle && direction == "RIGHT" {
        &sprites.jump[1]
    } else if player.almost_ground && !player.on_obstacle && direction == "LEFT" {
        &sprites.jump[4]
    }

    else if !player.on_ground && !player.on_obstacle && direction == "RIGHT" {
        &sprites.jump[2]
    } else if !player.on_ground && !player.on_obstacle && direction == "LEFT" {
        &sprites.jump[5]
    }

    else if direction == "RIGHT" {
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

    // Draw associated shadow if not on obstacle
    if !player.on_obstacle {
        draw_sprite(
            player.x as usize,
            GROUND as usize + 3,
            shadow_sprite,
            window_buffer,
            WINDOW_WIDTH
        );

    }
}

fn draw_background_sprite(sprites: &Sprites, buffer: &mut [u32], background_index: usize) {
    draw_sprite(0, 0, &sprites.background[background_index], buffer, WINDOW_WIDTH);
}