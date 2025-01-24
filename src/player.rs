use crate::my_ray;
use crate::render;
use crate::world::World;
use raylib::ffi;
use raylib::prelude::*;
use std::os::raw::c_int;

// TODO:
// 1. set discrete values for zoom
// 2. Get multiple rays into a fan out for the fov

pub enum RayInter {
    Vertical,
    Horizontal,
}

pub struct Player {
    pub pos: Vector2,
    velocity: f32,
    direction: Vector2, // cos and sin values
    angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector2::new(256.0, 256.0),
            angle: 0.0,
            velocity: 15.0,
            direction: Vector2::new(0.0, 0.0),
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, camera: &Camera2D, world: &mut World) {
        self.draw_direction_line(d, camera);
        // Draw the circle at the center of the tile
        let player_radius = 5.0 * camera.zoom;
        d.draw_circle(
            render::entity_to_screen(self.pos, camera).x as i32,
            render::entity_to_screen(self.pos, camera).y as i32,
            player_radius,
            Color::RED,
        );

        let ray_pos = my_ray::raycast_dda(self.pos, self.angle, world);
        // Convert the world space positions to screen space for drawing
        let ray_start_screen = render::entity_to_screen(self.pos, camera);
        let ray_end_screen = render::entity_to_screen(ray_pos, camera);

        // Draw the ray
        d.draw_line_ex(ray_start_screen, ray_end_screen, 5.0, Color::BLUE);
    }

    fn draw_direction_line(&self, d: &mut RaylibDrawHandle, camera: &Camera2D) {
        // Transform the player's position to screen space
        let player_screen_pos = render::entity_to_screen(self.pos, camera);

        // Calculate the end point of the direction line in world space
        let direction_line_end_world = Vector2::new(
            self.pos.x + self.direction.x * 25.0, // No zoom here, as this is in world space
            self.pos.y + self.direction.y * 25.0,
        );

        // Transform the end point to screen space
        let direction_line_end_screen = render::entity_to_screen(direction_line_end_world, camera);

        // Draw direction line in screen space
        d.draw_line_ex(
            player_screen_pos,
            direction_line_end_screen,
            2.0 * camera.zoom,
            Color::RED,
        );
    }

    pub fn input_update(&mut self, camera: &mut Camera2D) {
        unsafe {
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_W as c_int) {
                self.pos.x += self.direction.x * self.velocity;
                self.pos.y += self.direction.y * self.velocity;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_A as c_int) {
                self.angle -= 10.0;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_S as c_int) {
                self.pos.x -= self.direction.x * self.velocity;
                self.pos.y -= self.direction.y * self.velocity;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_D as c_int) {
                self.angle += 10.0;
            }
            if self.angle >= 360.0 {
                self.angle -= 360.0;
            }
            if self.angle < 0.0 {
                self.angle += 360.0;
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

            camera.target.x = self.pos.x;
            camera.target.y = self.pos.y;
            self.direction.x = self.angle.to_radians().cos();
            self.direction.y = self.angle.to_radians().sin();

            // Display the player's angle on the screen
            let angle_text = format!("Angle: {:.2}", self.angle);
            ffi::DrawText(
                angle_text.as_ptr() as *const u8, // Text pointer
                10,                               // X position
                10,                               // Y position
                20,                               // Font size
                ffi::Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                }, // White color
            );
        }
    }
}
