use crate::world::World;
use raylib::ffi;
use raylib::prelude::*;
use std::os::raw::c_int;

pub struct Player {
    pub x: f32,
    pub y: f32,
    velocity: f32,
    direction: Vector2,
    angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 200.0,
            y: 200.0,
            angle: 0.0,
            velocity: 2.5,
            direction: Vector2::new(0.0, 0.0),
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

        d.draw_line_ex(
            Vector2::new(self.x + 32.0, self.y + 32.0),
            Vector2::new(
                self.x + 32.0 + self.direction.x * 50.0,
                self.y + 32.0 + self.direction.y * 50.0,
            ),
            5.0,
            Color::RED,
        );
    }

    pub fn input_update(&mut self, world: &mut World) {
        unsafe {
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_W as c_int) {
                self.x += self.direction.x * self.velocity;
                self.y += self.direction.y * self.velocity;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_A as c_int) {
                self.angle -= 0.05;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_S as c_int) {
                self.x -= self.direction.x * self.velocity;
                self.y -= self.direction.y * self.velocity;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_D as c_int) {
                self.angle += 0.05;
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_L as c_int) {
                world.data = World::from_file("data.cade")
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to load world: {}", e);
                        World::new()
                    })
                    .data;
            }
            if self.angle >= 2.0 * std::f32::consts::PI {
                self.angle -= 2.0 * std::f32::consts::PI;
            }
            if self.angle < 0.0 {
                self.angle += 2.0 * std::f32::consts::PI;
            }

            self.direction.x = self.angle.cos();
            self.direction.y = self.angle.sin();
        }
    }
}
