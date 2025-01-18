// TODO:
// 1. animate the player directionally
// 2. Do some kind of collision detection
// 3. Load block data into render function

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 512;
const FPS: u32 = 64;

use player::Player;
use raylib::prelude::*;
use render::Renderer;
use std::io;
use world::World;

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
