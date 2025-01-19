use raylib::ffi;
use raylib::prelude::*;
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
            velocity: 5.0,
            direction: Vector2::new(0.0, 0.0),
            size: 64.0,
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D, camera: &Camera2D) {
        let world_pos = Vector2::new(self.x as f32, self.y as f32);

        let player_screen_pos = Self::entity_to_screen(world_pos, camera);

        let dest_rect = Rectangle {
            x: player_screen_pos.x.round(),
            y: player_screen_pos.y.round(),
            width: self.size * camera.zoom,
            height: self.size * camera.zoom,
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

        // Calculate the center of the player in screen space
        let player_center = Vector2::new(
            player_screen_pos.x + (self.size * camera.zoom) / 2.0,
            player_screen_pos.y + (self.size * camera.zoom) / 2.0,
        );

        // Calculate the end point of the direction line
        let direction_line_end = Vector2::new(
            player_center.x + self.direction.x * 50.0 * camera.zoom,
            player_center.y + self.direction.y * 50.0 * camera.zoom,
        );

        // Draw direction short line
        d.draw_line_ex(
            player_center,
            direction_line_end,
            5.0 * camera.zoom,
            Color::RED,
        );

        // Self::lerping(self, d, player_center);
    }

    fn entity_to_screen(entity_pos: Vector2, camera: &Camera2D) -> Vector2 {
        Vector2::new(
            (entity_pos.x - camera.target.x) * camera.zoom + camera.offset.x,
            (entity_pos.y - camera.target.y) * camera.zoom + camera.offset.y,
        )
    }

    // Alright we are raw-dogging this line drawing shit now

    fn lerping(&self, d: &mut RaylibDrawHandle, player_center: Vector2) {
        // Draw cirle at mouse position
        d.draw_circle_v(d.get_mouse_position(), 10.0, Color::BLUE);

        // Draw line to cirle at mouse position
        d.draw_line_ex(player_center, d.get_mouse_position(), 5.0, Color::RED);

        // Draw lerped circles between mouse and player
        Self::lerped_circles(self, d, 10, player_center);
    }

    fn lerped_circles(&self, d: &mut RaylibDrawHandle, num_circles: u8, player_center: Vector2) {
        for i in 1..num_circles {
            let lerp_t_value = i as f32 / num_circles as f32;
            let circle_pos: Vector2 =
                Vector2::lerp(&player_center, d.get_mouse_position(), lerp_t_value as f32);
            d.draw_circle_v(circle_pos, 10.0, Color::BLUE);
        }
    }

    pub fn input_update(&mut self, camera: &mut Camera2D) {
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

            // Camera zoom adjustments
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_R as i32) {
                camera.zoom -= 0.05;
                println!("Zoom: {:.2}", camera.zoom);
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_T as i32) {
                camera.zoom += 0.05;
                println!("Zoom: {:.2}", camera.zoom);
            }

            camera.target.x = self.x as f32;
            camera.target.y = self.y as f32;
            self.direction.x = self.angle.cos();
            self.direction.y = self.angle.sin();
        }
    }
}
