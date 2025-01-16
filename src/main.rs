// TODO:
// 1. animate the player directionally
// 2. Do some kind of collision detection

mod world {

    use raylib::prelude::*;

    #[derive(Copy, Clone)]
    pub enum Blocks {
        GRASS,
        STONE,
        PLAYER,
    }

    pub struct World {
        data: [[Blocks; 16]; 16],
    }

    impl World {
        pub fn new() -> Self {
            World {
                data: [[Blocks::STONE; 16]; 16],
            }
        }

        pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D) {
            for i in 0..self.data.len() {
                for j in 0..self.data[i].len() {
                    let dest_rect = Rectangle {
                        x: i as f32 * 64.0,
                        y: j as f32 * 64.0,
                        width: 64.0,
                        height: 64.0,
                    };

                    let mut texture_section = Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 32.0,
                        height: 32.0,
                    };

                    match self.data[i][j] {
                        Blocks::STONE => {
                            texture_section.x += 0.0;
                            texture_section.y += 32.0;
                        }
                        Blocks::GRASS => {
                            texture_section.x += 32.0;
                            texture_section.y += 32.0;
                        }

                        _ => texture_section.y += 32.0,
                    }

                    d.draw_texture_pro(
                        texture_atlas,
                        texture_section,
                        dest_rect,
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}

mod player {

    use raylib::ffi;
    use raylib::prelude::*;
    use std::os::raw::c_int;

    enum Direction {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }

    pub struct Player {
        x: f32,
        y: f32,
        velocity: f32,
        direction: Direction,
    }

    impl Player {
        pub fn new() -> Self {
            Player {
                x: 200.0,
                y: 200.0,
                velocity: 2.0,
                direction: Direction::UP,
            }
        }

        pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D) {
            let dest_rect = Rectangle {
                x: self.x,
                y: self.y,
                width: 64.0,
                height: 64.0,
            };

            let texture_section = Rectangle {
                x: 0.0,
                y: 0.0,
                width: 32.0,
                height: 32.0,
            };

            d.draw_texture_pro(
                texture_atlas,
                texture_section,
                dest_rect,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        pub fn mov(&mut self) {
            unsafe {
                if ffi::IsKeyDown(ffi::KeyboardKey::KEY_W as c_int) {
                    self.y -= self.velocity;
                }
                if ffi::IsKeyDown(ffi::KeyboardKey::KEY_A as c_int) {
                    self.x -= self.velocity;
                }
                if ffi::IsKeyDown(ffi::KeyboardKey::KEY_S as c_int) {
                    self.y += self.velocity;
                }
                if ffi::IsKeyDown(ffi::KeyboardKey::KEY_D as c_int) {
                    self.x += self.velocity;
                }
            }
        }
    }
}

use player::*;
use world::*;
use raylib::prelude::*;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(1024, 1024)
        .build();

    let texture_atlas = rl
        .load_texture(&thread, "./player_sheet.png")
        .expect("Failed to load texture");

    let mut player = Player::new();


    let world = World::new();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        player.mov();

        d.clear_background(Color::GRAY);
        world.render(&mut d, &texture_atlas);
        player.render(&mut d, &texture_atlas);

         
    }
}
