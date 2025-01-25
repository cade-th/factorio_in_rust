// TODO:
// 1. Do some kind of collision detection
// 2. Add load world function to the player
// 3. Something's wrong with the load from file function
// 4. Minimap Zoom is broken
// 5. World zoom works

const FPS: u32 = 20;

use player::Player;
use render::*;
use selector::Selector;
use state::State;
use world::World;

use raylib::prelude::*;
use std::io;

pub mod my_ray;
pub mod player;
pub mod render;
pub mod selector;
pub mod state;
pub mod world;

fn main() -> io::Result<()> {
    let mut state = State::new();

    let (mut rl, thread) = raylib::init()
        .size(state.screen_width, state.screen_height)
        .vsync()
        .build();

    let texture_atlas = rl
        .load_texture(&thread, "./player_sheet.png")
        .expect("Failed to load texture");

    let mut player = Player::new();
    let mut selector = Selector::new();

    /*
        // Load world from a file
        let world_result = World::from_file("data.cade");
        let mut world = match world_result {
            Ok(world) => world,
            Err(e) => {
                eprintln!("Error loading world: {}", e);
                return Err(e);
            }
        };
    */

    let mut world = World::new();

    let mut renderer = Renderer::new(RendererType::Minimap);

    rl.set_target_fps(FPS);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        state.update(&mut renderer, &mut player, &mut selector, &mut world);

        d.clear_background(Color::GRAY);
        renderer.render(
            &state,
            &mut d,
            &texture_atlas,
            &mut world,
            &selector,
            &player,
        )
    }

    Ok(())
}
