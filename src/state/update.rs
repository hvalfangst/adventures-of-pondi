use minifb::Key;

use crate::graphics::constants::*;
use crate::graphics::sprites::{draw_sprite, Sprite, Sprites};
use crate::state::*;
use crate::state::Direction::{Left, Right};
use crate::state::player::Player;

pub fn update(player: &mut Player, sprites: &Sprites, window_buffer: &mut Vec<u32>, background_index: usize) {
    draw_background_sprite(sprites, window_buffer, background_index);
    draw_player(sprites, window_buffer, player)
}

fn draw_player(sprites: &Sprites, window_buffer: &mut Vec<u32>, player: &mut Player) {

    // Determine the current direction and action of the player
    let direction = player.direction;
    let last_key = player.last_key.unwrap_or(Key::D);

    // Determine the sprite to draw
    let sprite_to_draw = if last_key == Key::X {
        if !player.is_kicking {
            player.is_kicking = true;
            player.kick_frame = 0;
            player.kick_frame_timer = 0;
        }

        if player.is_kicking {
            player.kick_frame_timer += 1;
            if player.kick_frame_timer >= KICK_FRAME_DURATION {
                player.kick_frame += 1;
                player.kick_frame_timer = 0;

                if player.kick_frame >= 2 {
                    player.is_kicking = false;
                    player.kick_frame = 0;
                }
            }
        }

        // Select the correct kick frame based on direction
        if player.is_kicking && direction == Right{
            &sprites.kick[player.kick_frame]
        } else if player.is_kicking && direction == Left {
            &sprites.kick[2 + player.kick_frame]
        } else {
            // Default to the player sprite if not kicking
            &sprites.player[player.right_increment]
        }
    }   else if player.almost_ground && !player.on_obstacle && direction == Right {
        &sprites.jump[1]
    } else if player.almost_ground && !player.on_obstacle && direction == Left {
        &sprites.jump[4]
    } else if !player.on_ground && !player.on_obstacle && direction == Right {
        &sprites.jump[2]
    } else if !player.on_ground && !player.on_obstacle && direction == Left {
        &sprites.jump[5]
    } else if direction == Right {
        &sprites.player[player.right_increment]
    } else if direction == Left {
        &sprites.player[player.left_increment]
    } else { // Default is moving to the right
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

    // Draw different sizes of shadows based on player state
    let shadow_sprite = if player.on_ground {
            &sprites.shadow[0]
    } else if player.almost_ground {
            &sprites.shadow[2]
    } else {
            &sprites.shadow[1]
    };

    // Draw associated shadow if not on or above obstacle
    if !player.on_obstacle && !player.above_obstacle {
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