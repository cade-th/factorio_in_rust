use crate::world::Blocks;
use crate::world::World;
use raylib::ffi;
use raylib::prelude::*;
use std::os::raw::c_int;

// TODO:
// 1. set discrete values for zoom
// Set squares and player and mouse to certain color

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

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        camera: &Camera2D,
        world: &World,
    ) {
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

        self.draw_ray_dda(d, player_center, world);

        self.draw_direction_line(d, player_center, camera);

        //self.draw_line_to_mouse(d, player_center, camera);

        // self.lerped_circles(d, 10, player_center, camera);
    }

    // Alright we are raw-dogging this line drawing shit now

    fn draw_ray_dda(&self, d: &mut RaylibDrawHandle, player_center: Vector2, world: &World) {
        // Should be player coordinates
        let ray_start = Vector2::new(
            self.x + world.tile_size as f32,
            self.y + world.tile_size as f32,
        ); // Starting point of the ray

        // Should be the player directin which is normalized (cos and sin)
        let ray_dir = self.direction;

        // Should be the sqaure the player is in
        // Maybe make a global variable of the square the player is in
        // Grid step direction
        let mut map_check = Vector2::new(
            (ray_start.x / world.tile_size as f32).floor(),
            (ray_start.y / world.tile_size as f32).floor(),
        );

        // Calculate unit step size
        // The distance the ray needs to travel along each axis to cross one grid cell:
        let ray_unit_step_size = Vector2::new((1.0 / ray_dir.x).abs(), (1.0 / ray_dir.y).abs());

        // Determine step direction and initial ray length
        let mut step = Vector2::new(0.0, 0.0);
        let mut ray_length_1d = Vector2::new(0.0, 0.0);

        if ray_dir.x < 0.0 {
            step.x = -1.0;
            ray_length_1d.x =
                (ray_start.x - map_check.x * world.tile_size as f32) * ray_unit_step_size.x;
        } else {
            step.x = 1.0;
            ray_length_1d.x =
                ((map_check.x + 1.0) * world.tile_size as f32 - ray_start.x) * ray_unit_step_size.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1.0;
            ray_length_1d.y =
                (ray_start.y - map_check.y * world.tile_size as f32) * ray_unit_step_size.y;
        } else {
            step.y = 1.0;
            ray_length_1d.y =
                ((map_check.y + 1.0) * world.tile_size as f32 - ray_start.y) * ray_unit_step_size.y;
        }

        // Perform "walk" until collision or max distance
        let max_distance = 100.0; // Limit the ray's distance
        let mut tile_found = false;
        let mut distance = 0.0;

        while !tile_found && distance < max_distance {
            if ray_length_1d.x < ray_length_1d.y {
                map_check.x += step.x;
                distance = ray_length_1d.x;
                ray_length_1d.x += ray_unit_step_size.x;
            } else {
                map_check.y += step.y;
                distance = ray_length_1d.y;
                ray_length_1d.y += ray_unit_step_size.y;
            }

            // Check if the current grid cell contains a wall
            let grid_x = map_check.x as i32;
            let grid_y = map_check.y as i32;

            if grid_x >= 0
                && grid_y >= 0
                && (grid_x as usize) < world.data.len()
                && (grid_y as usize) < world.data[0].len()
            {
                if world.data[grid_y as usize][grid_x as usize] == Blocks::STONE {
                    tile_found = true;
                }
            }
        }

        // Draw the ray
        let hit_point = ray_start + ray_dir * distance * world.tile_size as f32;

        d.draw_line_ex(
            player_center,
            hit_point,
            2.0,
            if tile_found {
                Color::YELLOW
            } else {
                Color::RED
            },
        );
    }

    fn draw_direction_line(
        &self,
        d: &mut RaylibDrawHandle,
        player_center: Vector2,
        camera: &Camera2D,
    ) {
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
    }

    pub fn input_update(&mut self, camera: &mut Camera2D) {
        unsafe {
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_W as c_int) {
                self.x += self.direction.x * self.velocity;
                self.y += self.direction.y * self.velocity;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_A as c_int) {
                self.angle -= 4.0;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_S as c_int) {
                self.x -= self.direction.x * self.velocity;
                self.y -= self.direction.y * self.velocity;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_D as c_int) {
                self.angle += 4.0;
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

            camera.target.x = self.x as f32;
            camera.target.y = self.y as f32;
            self.direction.x = self.angle.to_radians().cos();
            self.direction.y = self.angle.to_radians().sin();
        }
    }
    fn entity_to_screen(entity_pos: Vector2, camera: &Camera2D) -> Vector2 {
        Vector2::new(
            (entity_pos.x - camera.target.x) * camera.zoom + camera.offset.x,
            (entity_pos.y - camera.target.y) * camera.zoom + camera.offset.y,
        )
    }
}
