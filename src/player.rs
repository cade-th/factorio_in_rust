use raylib::ffi;
use raylib::prelude::*;
use std::ops::Add;
use std::os::raw::c_int;

pub struct Player {
    pub x: f32,
    pub y: f32,
    velocity: f32,
    direction: Vector2,
    angle: f32,
    size: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 200.0,
            y: 200.0,
            angle: 0.0,
            velocity: 2.5,
            direction: Vector2::new(0.0, 0.0),
            size: 64.0,
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D) {
        // TODO: Get this from World
        let tile_size = 64.0;

        let player_center: Vector2 = Vector2::new(
            self.x + self.size / 2.0 as f32,
            self.y + self.size / 2.0 as f32,
        );

        let mouse_grid: Vector2 = Vector2::new(
            d.get_mouse_x() as f32 / tile_size,
            d.get_mouse_y() as f32 / tile_size,
        );

        let dest_rect = Rectangle {
            x: self.x,
            y: self.y,
            width: self.size,
            height: self.size,
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

        // Draw direction short line
        d.draw_line_ex(
            player_center,
            Vector2::add(player_center, self.direction * 50.0),
            5.0,
            Color::RED,
        );

        Self::lerping(self, d, player_center);
    }

    // Alright we are raw-dogging this line drawing shit now

    fn lerping(&self, d: &mut RaylibDrawHandle, player_center: Vector2) {
        // Draw cirle at mouse position
        d.draw_circle_v(d.get_mouse_position(), 10.0, Color::BLUE);

        // Draw line to cirle at mouse position
        d.draw_line_ex(player_center, d.get_mouse_position(), 5.0, Color::RED);

        // Draw lerped circles between mouse and player
        Self::lerped_circles(self, d, 5, player_center);
    }

    fn lerped_circles(&self, d: &mut RaylibDrawHandle, num_circles: u8, player_center: Vector2) {
        for i in 1..num_circles {
            let lerp_t_value = i as f32 / num_circles as f32;
            let circle_pos: Vector2 =
                Vector2::lerp(&player_center, d.get_mouse_position(), lerp_t_value as f32);
            d.draw_circle_v(circle_pos, 10.0, Color::BLUE);
        }
    }

    pub fn input_update(&mut self) {
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
