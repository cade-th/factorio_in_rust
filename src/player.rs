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

const NUM_RAYS: i32 = 60; // This is also the fov

pub struct Player {
    pub pos: Vector2,
    velocity: f32,
    direction: Vector2, // cos and sin values
    angle: f32,
    distances: (Vec<f32>, Vec<Vector2>),
}

impl Player {
    pub fn new(world: &mut World) -> Self {
        let pos = Vector2::new(256.0, 256.0);
        let angle = 0.0;
        Player {
            pos,
            angle,
            velocity: 10.0,
            direction: Vector2::new(0.0, 0.0),
            distances: my_ray::cast_fov(pos, angle, NUM_RAYS, world),
        }
    }

    pub fn render(
        &self,
        state: &State,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        world: &World,
    ) {
        if state.view == View::FPS {
            Self::render_fps(self, state, d, world);
        } else {
            Self::render_minimap(self, d, camera);
        }
    }

    pub fn render_minimap(&self, d: &mut RaylibDrawHandle, camera: &Camera2D) {
        Self::draw_direction_line(self, d, camera);
        // Draw the circle at the center of the tile
        let player_radius = 5.0 * camera.zoom;
        d.draw_circle(
            render::entity_to_screen(self.pos, camera).x as i32,
            render::entity_to_screen(self.pos, camera).y as i32,
            player_radius,
            Color::RED,
        );

        for i in 0..self.distances.1.len() {
            let ray_pos = self.distances.1[i];
            // Convert the world space positions to screen space for drawing
            let ray_start_screen = render::entity_to_screen(self.pos, camera);
            let ray_end_screen = render::entity_to_screen(ray_pos, camera);

            // Draw the ray
            d.draw_line_ex(ray_start_screen, ray_end_screen, 2.0, Color::BLUE);
        }
    }

    pub fn render_fps(&self, state: &State, d: &mut RaylibDrawHandle, world: &World) {
        let num_rays = self.distances.0.len();
        let column_width = (state.screen_width / num_rays as i32) as i32;

        for i in 0..num_rays {
            let wall_height =
                (state.screen_height as f32 * world.tile_size as f32) / self.distances.0[i];
            let x = i as i32 * column_width;
            let y = ((state.screen_height as f32 - wall_height) / 2.0) as i32;
            let height = wall_height as i32;

            if self.distances.0[i] < 1000.0 {
                d.draw_rectangle(x, y, column_width, height, Color::BLUE);
            }
        }
    }

    pub fn input_update(&mut self, camera: &mut Camera2D, state: &mut State, world: &World) {
        unsafe {
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_W as c_int) {
                self.pos.x += self.direction.x * self.velocity;
                self.pos.y += self.direction.y * self.velocity;
                self.distances = my_ray::cast_fov(self.pos, self.angle, NUM_RAYS, world);
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_A as c_int) {
                self.angle -= 10.0;
                self.distances = my_ray::cast_fov(self.pos, self.angle, NUM_RAYS, world);
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_S as c_int) {
                self.pos.x -= self.direction.x * self.velocity;
                self.pos.y -= self.direction.y * self.velocity;
                self.distances = my_ray::cast_fov(self.pos, self.angle, NUM_RAYS, world);
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_D as c_int) {
                self.angle += 10.0;
                self.distances = my_ray::cast_fov(self.pos, self.angle, NUM_RAYS, world);
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
