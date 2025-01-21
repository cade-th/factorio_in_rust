use crate::world::Blocks;
use crate::world::World;
use raylib::ffi;
use raylib::prelude::*;
use std::os::raw::c_int;

// TODO:
// 1. set discrete values for zoom
// 2. Get distance from brute force raycast
// 3. Set DDA to same prototype as brute force
// 4. Show squares that the dda is going into and finish it

pub struct Player {
    pub pos: Vector2,
    velocity: f32,
    direction: Vector2,
    angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector2::new(256.0, 256.0),
            angle: 0.0,
            velocity: 5.0,
            direction: Vector2::new(0.0, 0.0),
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, camera: &Camera2D, world: &mut World) {
        let player_screen_pos = Self::entity_to_screen(self.pos, camera);

        // Draw the circle at the center of the tile
        let player_radius = 10.0 * camera.zoom;
        d.draw_circle(
            player_screen_pos.x as i32,
            player_screen_pos.y as i32,
            player_radius,
            Color::BLUE,
        );

        // self.render_current_cell(d, world, player_world, camera);

        self.draw_direction_line(d, camera);

        self.ray_cast_brute_force(d, world, camera);
    }

    pub fn ray_cast_brute_force(
        &self,
        d: &mut RaylibDrawHandle,
        world: &mut World,
        camera: &Camera2D,
    ) {
        // Start the ray in world space at the player's position
        let mut ray_end = self.pos;
        let step_size: f32 = 2.0;

        // World grid dimensions
        let grid_width = world.data.len();
        let grid_height = world.data[0].len();

        // Check if the ray is within bounds and not hitting a wall
        while ray_end.x >= 0.0
            && ray_end.y >= 0.0
            && ((ray_end.x / world.tile_size as f32) as usize) < grid_width
            && ((ray_end.y / world.tile_size as f32) as usize) < grid_height
            && world.data[(ray_end.x / world.tile_size as f32) as usize]
                [(ray_end.y / world.tile_size as f32) as usize]
                != Blocks::STONE
        {
            ray_end.x += self.direction.x * step_size;
            ray_end.y += self.direction.y * step_size;
        }

        // Convert the world space positions to screen space for drawing
        let ray_start_screen = Self::entity_to_screen(self.pos, camera);
        let ray_end_screen = Self::entity_to_screen(ray_end, camera);

        // Draw the ray
        d.draw_line_ex(ray_start_screen, ray_end_screen, 5.0, Color::RED);
    }

    fn entity_to_screen(entity_pos: Vector2, camera: &Camera2D) -> Vector2 {
        Vector2::new(
            (entity_pos.x - camera.target.x) * camera.zoom + camera.offset.x,
            (entity_pos.y - camera.target.y) * camera.zoom + camera.offset.y,
        )
    }

    // This works now
    pub fn player_to_world(&self, world: &World) -> Vector2 {
        Vector2::new(
            (self.pos.x / world.tile_size as f32).floor(),
            (self.pos.y / world.tile_size as f32).floor(),
        )
    }

    pub fn render_current_cell(
        &self,
        d: &mut RaylibDrawHandle,
        world: &World,
        player_world: Vector2,
        camera: &Camera2D,
    ) {
        // Calculate the grid cell the player is in
        let grid_cell_top_left_world = Vector2::new(
            (player_world.x as f32) * world.tile_size as f32,
            (player_world.y as f32) * world.tile_size as f32,
        );

        // Transform the grid cell's top-left corner to screen space
        let grid_cell_screen_top_left = Self::entity_to_screen(grid_cell_top_left_world, camera);

        // Draw the blue square representing the grid cell
        d.draw_rectangle(
            grid_cell_screen_top_left.x as i32,
            grid_cell_screen_top_left.y as i32,
            world.tile_size as i32,
            world.tile_size as i32,
            Color::BLUE, // Semi-transparent blue
        );
    }

    fn draw_direction_line(&self, d: &mut RaylibDrawHandle, camera: &Camera2D) {
        // Transform the player's position to screen space
        let player_screen_pos = Self::entity_to_screen(self.pos, camera);

        // Calculate the end point of the direction line in world space
        let direction_line_end_world = Vector2::new(
            self.pos.x + self.direction.x * 50.0, // No zoom here, as this is in world space
            self.pos.y + self.direction.y * 50.0,
        );

        // Transform the end point to screen space
        let direction_line_end_screen = Self::entity_to_screen(direction_line_end_world, camera);

        // Draw direction line in screen space
        d.draw_line_ex(
            player_screen_pos,
            direction_line_end_screen,
            5.0 * camera.zoom,
            Color::RED,
        );
    }

    // Alright we are raw-dogging this line drawing shit now

    fn draw_ray_dda(&self, d: &mut RaylibDrawHandle, player_center: Vector2, world: &World) {
        // Should be player coordinates
        let ray_start = Vector2::new(
            self.pos.x + world.tile_size as f32,
            self.pos.y + world.tile_size as f32,
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

    pub fn input_update(&mut self, camera: &mut Camera2D) {
        unsafe {
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_W as c_int) {
                self.pos.x += self.direction.x * self.velocity;
                self.pos.y += self.direction.y * self.velocity;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_A as c_int) {
                self.angle -= 4.0;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_S as c_int) {
                self.pos.x -= self.direction.x * self.velocity;
                self.pos.y -= self.direction.y * self.velocity;
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

            camera.target.x = self.pos.x;
            camera.target.y = self.pos.y;
            self.direction.x = self.angle.to_radians().cos();
            self.direction.y = self.angle.to_radians().sin();
        }
    }
}
