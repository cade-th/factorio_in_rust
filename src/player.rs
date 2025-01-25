use crate::my_ray;
use crate::player;
use crate::render;
use crate::state::*;
use crate::world;
use crate::world::*;
use raylib::ffi;
use raylib::prelude::*;
use std::os::raw::c_int;

// TODO:
// 1. set discrete values for zoom
// 2. Only draw rays when player moves to put less stress on the cpu

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
            velocity: 10.0,
            direction: Vector2::new(0.0, 0.0),
        }
    }

    pub fn render(
        &self,
        state: &State,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        world: &mut World,
    ) {
        // Move this somewhere
        let distances = my_ray::cast_fov(self.pos, self.angle, 60.0, 150, world).1;

        if state.view == View::FPS {
            Self::render_fps(self, distances, state, d);
        } else {
            Self::render_minimap(self, distances, d, camera);
        }
    }

    pub fn render_minimap(
        &self,
        distances: Vec<Vector2>,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
    ) {
        Self::draw_direction_line(self, d, camera);
        // Draw the circle at the center of the tile
        let player_radius = 5.0 * camera.zoom;
        d.draw_circle(
            render::entity_to_screen(self.pos, camera).x as i32,
            render::entity_to_screen(self.pos, camera).y as i32,
            player_radius,
            Color::RED,
        );

        for i in 0..distances.len() {
            let ray_pos = distances[i];
            // Convert the world space positions to screen space for drawing
            let ray_start_screen = render::entity_to_screen(self.pos, camera);
            let ray_end_screen = render::entity_to_screen(ray_pos, camera);

            // Draw the ray
            d.draw_line_ex(ray_start_screen, ray_end_screen, 2.0, Color::BLUE);
        }

        /*
         */
    }

    pub fn render_fps(&self, distances: Vec<Vector2>, state: &State, d: &mut RaylibDrawHandle) {
        let screen_width = state.screen_width;
        let screen_height = state.screen_height;
        let num_rays = distances.len();
        let column_width = screen_width as f32 / num_rays as f32;

        for (i, hit_point) in distances.iter().enumerate() {
            // Calculate distance to the wall
            let dx = hit_point.x - self.pos.x;
            let dy = hit_point.y - self.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Correct for fish-eye effect by multiplying by the cosine of the ray angle
            let angle_offset = (i as f32 / num_rays as f32 - 0.5) * 60.0_f32.to_radians();
            let corrected_distance = distance * angle_offset.cos();

            // Calculate wall height relative to corrected distance
            let wall_height = (screen_height as f32 * 50.0) / corrected_distance;

            // Determine the color intensity based on the distance (farther = darker)
            let intensity = (1.0 - (corrected_distance / 500.0).min(1.0)) * 255.0;
            let wall_color = Color::new(intensity as u8, intensity as u8, intensity as u8, 255);

            // Calculate the screen coordinates of the wall slice
            let x = (i as f32 * column_width) as i32;
            let y = ((screen_height as f32 - wall_height) / 2.0) as i32;
            let height = wall_height as i32;

            // Draw the wall slice
            d.draw_rectangle(x, y, column_width as i32, height, wall_color);
        }
    }

    pub fn input_update(&mut self, camera: &mut Camera2D, state: &mut State, world: &World) {
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

            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_U as i32) {
                state.change_view(View::Minimap);
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_I as i32) {
                state.change_view(View::Editor);
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_O as i32) {
                state.change_view(View::FPS);
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_G as i32) {
                let _ = world::World::from_file("data.cade");
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
}
