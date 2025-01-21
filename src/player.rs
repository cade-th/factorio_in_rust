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
    direction: Vector2, // cos and sin values
    angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector2::new(256.0, 256.0),
            angle: 45.0,
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

        // self.draw_direction_line(d, camera);

        // self.ray_cast_brute_force(d, world, camera);
        self.ray_cast_dda(d, world, camera);
    }

    pub fn ray_cast_dda(
        &self,
        d: &mut RaylibDrawHandle,
        world: &mut World,
        camera: &Camera2D,
    ) -> Vector2 {
        // Starting point of the ray in world space
        let mut ray_end = self.pos;

        // Ray direction (assumed normalized)
        let ray_angle = self.angle;
        let ray_direction = self.direction;

        // Step direction: adding 1 or -1 in each direction
        let mut step = Vector2::new(0.0, 0.0);

        // Get the current cell we're in
        let mut current_cell = Vector2::new(
            (ray_end.x / world.tile_size as f32).floor(),
            (ray_end.y / world.tile_size as f32).floor(),
        );

        // Calculate unit step size
        let unit_step_size = Vector2::new(
            (1.0 / ray_direction.x).abs(), // distance to the next vertical grid line
            (1.0 / ray_direction.y).abs(), // distance to the next horizontal grid line
        );

        // Calculate the length of the ray to the first intersection
        let mut ray_length = Vector2::new(0.0, 0.0);

        if ray_direction.x < 0.0 {
            ray_length.x =
                ((current_cell.x * world.tile_size as f32) - ray_end.x).abs() * unit_step_size.x;
        } else {
            ray_length.x = (((current_cell.x + 1.0) * world.tile_size as f32) - ray_end.x).abs()
                * unit_step_size.x;
        }

        if ray_direction.y < 0.0 {
            ray_length.y =
                ((current_cell.y * world.tile_size as f32) - ray_end.y).abs() * unit_step_size.y;
        } else {
            ray_length.y = (((current_cell.y + 1.0) * world.tile_size as f32) - ray_end.y).abs()
                * unit_step_size.y;
        }

        // Calculate the intersection points along the ray's path
        let intersection_x = Vector2::new(
            ray_end.x + ray_direction.x * ray_length.x,
            ray_end.y + ray_direction.y * ray_length.x,
        );

        let intersection_y = Vector2::new(
            ray_end.x + ray_direction.x * ray_length.y,
            ray_end.y + ray_direction.y * ray_length.y,
        );

        // Convert the intersection points to screen space
        let player_screen_pos = Self::entity_to_screen(self.pos, camera);
        let intersection_x_screen = Self::entity_to_screen(intersection_x, camera);
        let intersection_y_screen = Self::entity_to_screen(intersection_y, camera);

        // Draw lines along the ray's path to these intersection points
        d.draw_line_ex(
            player_screen_pos,
            intersection_x_screen,
            5.0,
            Color::GREEN, // Line along the ray for x-axis intersection
        );

        d.draw_line_ex(
            player_screen_pos,
            intersection_y_screen,
            2.0,
            Color::YELLOW, // Line along the ray for y-axis intersection
        );

        // Draw final ray
        d.draw_line_ex(
            Self::entity_to_screen(self.pos, camera),
            Self::entity_to_screen(ray_end, camera),
            5.0,
            Color::BLUE,
        );

        ray_end
    }

    fn draw_ray_lengths(
        d: &mut RaylibDrawHandle,
        ray_start: Vector2,
        ray_direction: Vector2,
        ray_length: Vector2,
        camera: &Camera2D,
    ) {
        // Calculate the endpoint for the horizontal (x-axis) ray length
        let ray_length_x_end =
            Vector2::new(ray_start.x + ray_direction.x * ray_length.x, ray_start.y);

        // Calculate the endpoint for the vertical (y-axis) ray length
        let ray_length_y_end =
            Vector2::new(ray_start.x, ray_start.y + ray_direction.y * ray_length.y);

        // Convert world positions to screen positions
        let ray_start_screen = Self::entity_to_screen(ray_start, camera);
        let ray_length_x_screen = Self::entity_to_screen(ray_length_x_end, camera);
        let ray_length_y_screen = Self::entity_to_screen(ray_length_y_end, camera);

        // Draw the horizontal (x-axis) ray length
        d.draw_line_ex(
            ray_start_screen,
            ray_length_x_screen,
            2.0,
            Color::GREEN, // Horizontal ray length in green
        );

        // Draw the vertical (y-axis) ray length
        d.draw_line_ex(
            ray_start_screen,
            ray_length_y_screen,
            2.0,
            Color::YELLOW, // Vertical ray length in yellow
        );
    }

    pub fn get_quadrant(angle: f32) -> Vector2 {
        let mut return_vec = Vector2::new(0.0, 0.0);

        // Determine which quadrant the ray is in
        if angle >= 0.0 && angle <= 180.0 {
            return_vec.y = 1.0;
        } else {
            return_vec.y = -1.0;
        }

        if angle >= 90.0 && angle <= 270.0 {
            return_vec.x = -1.0;
        } else {
            return_vec.x = 1.0;
        }

        return_vec
    }

    pub fn ray_cast_brute_force(
        &self,
        d: &mut RaylibDrawHandle,
        world: &mut World,
        camera: &Camera2D,
    ) -> Vector2 {
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

        return ray_end;
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
