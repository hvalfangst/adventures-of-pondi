use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::state::{GRAVITY, GROUND, jump_obstacles, LOWER_BOUND, Obstacle, UPPER_BOUND};
use crate::state::player::Player;

pub trait GlobalCommand {
    fn execute(&self, player: &mut Player, obstacles: &Vec<Obstacle>);
}

pub struct ApplyGravity;

impl GlobalCommand for ApplyGravity {
    fn execute(&self, player: &mut Player, _obstacles: &Vec<Obstacle>) {
        if !player.on_ground && !player.on_obstacle {
            player.vy += GRAVITY;
        }
    }
}

pub struct JumpingObstacles;

impl GlobalCommand for JumpingObstacles {
    fn execute(&self, player: &mut Player, obstacles: &Vec<Obstacle>) {
        jump_obstacles(player, obstacles);
    }
}

pub struct VerticalBounds;

impl GlobalCommand for VerticalBounds {
    fn execute(&self, player: &mut Player, _obstacles: &Vec<Obstacle>) {
        // Prevent the player from moving out vertical (y) bounds
        if player.y <= 40.0 {
            player.on_ground = false;
            player.y = GROUND;
        }
    }
}

pub struct HorizontalBounds;

impl GlobalCommand for HorizontalBounds {
    fn execute(&self, player: &mut Player, _obstacles: &Vec<Obstacle>) {
        // Prevent the player from moving out horizontal (x) bounds
        if player.x < LOWER_BOUND {
            player.x = LOWER_BOUND;
            player.vx = 0.0;
        } else if player.x > UPPER_BOUND {
            player.x = UPPER_BOUND;
            player.vx = 0.0;
        }
    }
}

pub fn initialize_global_command_map() -> HashMap<String, Rc<RefCell<dyn GlobalCommand>>> {
    let mut global_commands: HashMap<String, Rc<RefCell<dyn GlobalCommand>>> = HashMap::new();
    global_commands.insert("JumpingObstacles".to_string(), Rc::new(RefCell::new(JumpingObstacles)));
    global_commands.insert("ApplyGravity".to_string(), Rc::new(RefCell::new(ApplyGravity)));
    global_commands.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(VerticalBounds)));
    global_commands.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(HorizontalBounds)));


    global_commands
}

fn is_on_obstacle(player_x: f32, player_y: f32, obstacle: &Obstacle) -> bool {
    player_x > obstacle.x_left && player_x < obstacle.x_right &&
        player_y > obstacle.y_top && player_y < obstacle.y_bottom
}