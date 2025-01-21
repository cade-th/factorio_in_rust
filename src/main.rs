// TODO:
// 1. Get raycasting working via interpolation
// 2. Do some kind of collision detection

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 1024;
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
        .vsync()
        .build();

    let texture_atlas = rl
        .load_texture(&thread, "./player_sheet.png")
        .expect("Failed to load texture");

    let mut player = Player::new();

    // Load world from a file
    let world_result = World::from_file("data.cade");
    let mut world = match world_result {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error loading world: {}", e);
            return Err(e);
        }
    };

    let mut renderer = Renderer::new(&player);

    rl.set_target_fps(FPS);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        player.input_update(&mut renderer.camera);

        d.clear_background(Color::GRAY);
        renderer.render(&mut d, &texture_atlas, &mut world, &player);
    }

    Ok(())
}
