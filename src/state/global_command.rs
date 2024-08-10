use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::state::{GRAVITY, GROUND, LOWER_BOUND, Obstacle, UPPER_BOUND};
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

pub struct CollisionDetection;

impl GlobalCommand for CollisionDetection {
    fn execute(&self, player: &mut Player, obstacles: &Vec<Obstacle>) {
        let mut obstacles_on = HashSet::new(); // Track obstacles the player is on
        let mut highest_y_position = player.y; // Track the highest y position for transition

        for obs in obstacles.iter() {
            if is_on_obstacle(player.x, player.y, obs) {
                println!("Player is on obstacle: {:?}", obs.id);
                obstacles_on.insert(obs.id);
                if obs.y_transition_pos > highest_y_position {
                    highest_y_position = obs.y_transition_pos;
                }
            } else {
                println!("Player is not on obstacle: {:?}", obs.id);
            }
        }

        // Update player's position and state based on obstacles tracked
        player.y = highest_y_position;
        player.on_obstacles = obstacles_on;
        // player.on_ground = !player.on_obstacles.is_empty(); // Set on ground if on any obstacle

        println!("Final player state:");
        println!("Player position: x = {}, y = {}", player.x, player.y);
        println!("Obstacles player is on: {:?}", player.on_obstacles);
        println!("Player on ground: {}", player.on_ground);
        println!("Highest y transition position: {}\n", highest_y_position);
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

pub struct Misc;

impl GlobalCommand for Misc {
    fn execute(&self, player: &mut Player, _obstacles: &Vec<Obstacle>) {

        // Apply vertical velocity
        if !player.on_obstacle {
            player.y += player.vy;
        }

        if player.y >= 140.0 && player.y <= 160.0   {
            player.almost_ground = true;
        } else {
            player.almost_ground = false;
        }

        // Reset vertical velocity and flag if player is on the ground
        if player.y >= GROUND {
            player.on_ground = true;
            player.almost_ground = false;
            player.vy = 0.0;
            player.y = GROUND;
        } else {
            player.on_ground = false;
        }
    }
}

pub fn initialize_global_command_map() -> HashMap<String, Rc<RefCell<dyn GlobalCommand>>> {
    let mut global_commands: HashMap<String, Rc<RefCell<dyn GlobalCommand>>> = HashMap::new();

    global_commands.insert("ApplyGravity".to_string(), Rc::new(RefCell::new(ApplyGravity)));
    global_commands.insert("CollisionDetection".to_string(), Rc::new(RefCell::new(CollisionDetection)));
    global_commands.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(VerticalBounds)));
    global_commands.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(HorizontalBounds)));
    global_commands.insert("Misc".to_string(), Rc::new(RefCell::new(Misc)));

    global_commands
}

fn is_on_obstacle(player_x: f32, player_y: f32, obstacle: &Obstacle) -> bool {
    player_x > obstacle.x_left && player_x < obstacle.x_right &&
        player_y > obstacle.y_top && player_y < obstacle.y_bottom
}