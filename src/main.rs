// TODO:
// 1. Get raycasting working via interpolation
// 2. Do some kind of collision detection

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 512;
const FPS: u32 = 64;

use player::Player;
use render::Renderer;
use world::World;

use raylib::prelude::*;
use std::io;

pub mod player;
pub mod render;
pub mod world;

fn main() -> io::Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .build();

    let texture_atlas = rl
        .load_texture(&thread, "./player_sheet.png")
        .expect("Failed to load texture");

    let mut player = Player::new();

    let world_result = World::new();
    // This is needed cuz we're getting it from a file
    let world = match world_result {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error creating world: {}", e);
            return Err(e);
        }
    };
    let renderer = Renderer::new();

    rl.set_target_fps(FPS);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        player.input_update();

        d.clear_background(Color::GRAY);
        renderer.render(&mut d, &texture_atlas, &world, &player);
    }

    Ok(())
}
