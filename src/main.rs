use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use minifb::{Window, WindowOptions};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;

use crate::{
    graphics::sprites::Sprites,
    state::event_loop::start_event_loop
};
use crate::graphics::constants::{SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};
use crate::state::{GameState, Map, Obstacle, ObstacleId, Viewport};
use crate::state::command::initialize_command_map;
use crate::state::global_command::initialize_global_command_map;
use crate::state::player::Player;
use rodio::{Decoder, OutputStream, Sink};

mod state;mod graphics;



fn main() {
    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();

    let sprites = Sprites::new();
    let mut player = Player::new(1.0, 176.0);
    let commands = initialize_command_map();
    let global_commands = initialize_global_command_map();

    let (mut map_one_tiles, map_one_width, map_one_height) = read_grid_from_file("map_one.txt").expect("Failed to read grid from file");
    let (mut map_two_tiles, map_two_width, map_two_height) = read_grid_from_file("map_two.txt").expect("Failed to read grid from file");
    let (mut map_three_tiles, map_two_width, map_two_height) = read_grid_from_file("map_three.txt").expect("Failed to read grid from file");
    let mut map_one_obstacles = extract_obstacles(&map_one_tiles, false);
    let mut map_two_obstacles = extract_obstacles(&map_two_tiles, false);
    let mut map_three_obstacles = extract_obstacles(&map_three_tiles, false);


    let fullscreen = false;

    // Determine window size based on fullscreen flag
    let (window_width, window_height) = if fullscreen {
        let primary_monitor: MonitorHandle =  EventLoop::new().primary_monitor().expect("Failed to get primary monitor");
        let screen_size = primary_monitor.size();
        (screen_size.width as usize, screen_size.height as usize)
    } else {
        (SCALED_WINDOW_WIDTH, SCALED_WINDOW_HEIGHT)
    };

    // Create a window with the dimensions of the primary monitor
    let mut window = Window::new(
        "Age of Panda",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Initialize window and scaled buffer
    let mut window_buffer = vec![0; map_one_width * map_one_height];
    let mut scaled_buffer = vec![0; window_width * window_height];

    let map_one = Map {
        id: 1,
        tiles: map_one_tiles,
        obstacles: &mut map_one_obstacles,
        width: map_one_width,
        height: map_one_height,
        starting_x: 0.0,
        starting_y: 0.0,
        transition_x: 200.0,
        transition_y: 0.0
    };

    let map_two = Map {
        id: 2,
        tiles: map_two_tiles,
        obstacles: &mut map_two_obstacles,
        width: map_two_width,
        height: map_two_height,
        starting_x: 0.0,
        starting_y: 0.0,
        transition_x: 200.0,
        transition_y: 0.0
    };

    let map_three = Map {
        id: 3,
        tiles: map_three_tiles,
        obstacles: &mut map_three_obstacles,
        width: map_two_width,
        height: map_two_height,
        starting_x: 0.0,
        starting_y: 0.0,
        transition_x: 200.0,
        transition_y: 0.0
    };

    let all_maps = vec![map_one, map_two, map_three];

    let sounds: Vec<Vec<u8>> = load_sounds();

    let game_state = GameState {
        player,
        sprites,
        window_buffer: &mut window_buffer,
        grass_sprite_index: 0,
        sky_sprite_index: 0,
        window_width,
        window_height,
        window: &mut window,
        scaled_buffer: &mut scaled_buffer,
        game_over_index: 0,
        viewport: Viewport::new(window_width as f32, window_height as f32),
        all_maps,
        current_map_index: 0,
        footstep_index: 0,
        footstep_active: false,
        sounds
    };

    start_event_loop(game_state, commands, global_commands, &mut sink);
}

fn load_sound(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open sounds file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    buffer
}


fn load_sounds() -> Vec<Vec<u8>> {
    let mut sounds = Vec::new();
    sounds.push(load_sound("assets/sounds/walk_1.wav"));
    sounds.push(load_sound("assets/sounds/walk_2.wav"));
    sounds.push(load_sound("assets/sounds/walk_3.wav"));
    sounds.push(load_sound("assets/sounds/walk_4.wav"));
    sounds.push(load_sound("assets/sounds/jump.wav"));
    sounds.push(load_sound("assets/sounds/fall_mild.wav"));
    sounds.push(load_sound("assets/sounds/fall_heavy.wav"));
    sounds.push(load_sound("assets/sounds/down.wav"));
    sounds.push(load_sound("assets/sounds/explosion.wav"));
    sounds.push(load_sound("assets/sounds/kick.wav"));
    sounds.push(load_sound("assets/sounds/kick_box.wav"));
    sounds
}

fn read_grid_from_file(filename: &str) -> io::Result<(Vec<Tile>, usize, usize)> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line?.trim().to_string();
        for (x, c) in line.split_whitespace().enumerate() {
            let x_left = x as f32 * 16.0;
            let x_right = x_left + 16.0;
            let y_bottom = y as f32 * 16.0;
            let y_top = y_bottom - 16.0;
            let tile_type = match c {
                "X" => TileType::Obstacle,
                "G" => TileType::Grass,
                "O" => TileType::Sky,
                _ => TileType::Unknown,
            };
            grid.push(Tile {
                tile_type,
                x_left,
                x_right,
                y_bottom,
                y_top,
            });
        }
    }

    // Automatically detect resolution based on grid size
    let (width, height) = if !grid.is_empty() {
        let width = grid.iter().map(|tile| tile.x_right).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0) as usize;
        let height = grid.iter().map(|tile| tile.y_bottom).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0) as usize;

        println!("Detected resolution: {}x{}", width, height);
        (width, height)
    } else {
        (0, 0)
    };

    Ok((grid, width, height))
}

fn extract_obstacles(grid: &Vec<Tile>, sort_by_y: bool) -> Vec<Obstacle> {
    let mut obstacles = Vec::new();
    for tile in grid {
        if let TileType::Obstacle = tile.tile_type {
            obstacles.push(Obstacle {
                id: ObstacleId(obstacles.len()),
                x_left: tile.x_left,
                x_right: tile.x_right,
                y_bottom: tile.y_bottom,
                y_top: tile.y_top,
                active: true,
                durability: 2,
                falling: false,
                velocity_y: 0.0
            });
        }
    }

    // if sort_by_y {
    //     sort_obstacles_by_y(&mut obstacles);
    // }

    obstacles
}

pub fn sort_obstacles_by_y(mut obstacles: Vec<Obstacle>) -> Vec<Obstacle> {
        // Sort by Y position
        for i in 1..obstacles.len() {
            let mut j = i;
            while j > 0 && obstacles[j - 1].y_bottom < obstacles[j].y_bottom {
                obstacles.swap(j, j - 1);
                j -= 1;
            }
        }
    obstacles
}

fn print_grid(grid: &Vec<Tile>) {
    let mut grid_2d: Vec<Vec<String>> = vec![vec![String::new(); 16]; 16];
    for tile in grid {
        let x = (tile.x_left / 16.0) as usize;
        let y = (tile.y_bottom / 16.0) as usize;
        grid_2d[y][x] = match tile.tile_type {
            TileType::Obstacle => "X".to_string(),
            TileType::Grass => "G".to_string(),
            TileType::Sky => "O".to_string(),
            TileType::Unknown => "?".to_string(),
        };
    }
    for row in grid_2d {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

/// Searches for obstacles in the grid and outputs their positions.
fn find_obstacles(grid: &Vec<Tile>) {
    for tile in grid {
        if let TileType::Obstacle = tile.tile_type {
            // println!("Obstacle found: X.LEFT {}, X.RIGHT {}, Y.BOTTOM {}, Y.TOP {}", tile.x_left, tile.x_right, tile.y_bottom, tile.y_top);
        }
    }
}

#[derive(Debug)]
pub enum TileType {
    Obstacle,
    Grass,
    Sky,
    Unknown,
}

#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
    x_left: f32,
    x_right: f32,
    y_bottom: f32,
    y_top: f32,
}