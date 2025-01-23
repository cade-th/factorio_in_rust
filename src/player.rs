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
            velocity: 5.0,
            direction: Vector2::new(0.0, 0.0),
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, camera: &Camera2D, world: &mut World) {
        self.draw_direction_line(d, camera);
        // Draw the circle at the center of the tile
        let player_radius = 5.0 * camera.zoom;
        d.draw_circle(
            Self::entity_to_screen(&self.pos, camera).x as i32,
            Self::entity_to_screen(&self.pos, camera).y as i32,
            player_radius,
            Color::RED,
        );

        Self::raycast_dda(self, d, camera, world);
        // self.ray_cast_brute_force(d, world, camera);
    }

    pub fn raycast_dda(&self, d: &mut RaylibDrawHandle, camera: &Camera2D, world: &World) {
        // Get the current cell that the ray is inside
        let mut grid_pos = Self::entity_to_world(&self.pos, world);

        // Determine the direction from that
        //right
        grid_pos.x += if self.angle.to_radians().cos() > 0.0 {
            1.0
        //left
        } else {
            0.0
        };
        //up
        grid_pos.y += if self.angle.to_radians().sin() > 0.0 {
            1.0
        //down
        } else {
            0.0
        };

        // Turn the cell coordinates entity coordinates
        grid_pos = Self::world_to_entity(grid_pos, world);

        let intersection_x = self.intersection_point_x(grid_pos);
        let intersection_y = self.intersection_point_y(grid_pos);

        let result = self.inter_section_point(d, camera, &grid_pos, intersection_x, intersection_y);
    }

    pub fn inter_section_point(
        &self,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        grid_pos: &Vector2,
        input_x: (f32, Vector2),
        input_y: (f32, Vector2),
    ) -> (Vector2, bool) {
        if input_x.0 < input_y.0 {
            d.draw_circle(
                Self::entity_to_screen(&input_x.1, camera).x as i32,
                Self::entity_to_screen(&input_x.1, camera).y as i32,
                8.0,
                Color::GREEN,
            );
            return (input_x.1, true);
        } else {
            d.draw_circle(
                Self::entity_to_screen(&input_y.1, camera).x as i32,
                Self::entity_to_screen(&input_y.1, camera).y as i32,
                8.0,
                Color::GREEN,
            );
            return (input_y.1, false);
        }
    }

    pub fn intersection_point_x(&self, grid_pos: Vector2) -> (f32, Vector2) {
        let adjacent = grid_pos.x - self.pos.x;

        let hypotenuse = adjacent / self.angle.to_radians().cos();

        let b = hypotenuse * self.angle.to_radians().sin();

        let y = self.pos.y + b;

        let final_vec = Vector2::new(grid_pos.x, y);

        /*
        d.draw_circle(
            Self::entity_to_screen(&final_vec, &camera).x as i32,
            Self::entity_to_screen(&final_vec, &camera).y as i32,
            5.0,
            Color::BLUE,
        );
        */

        (hypotenuse, final_vec)
    }

    pub fn intersection_point_y(&self, grid_pos: Vector2) -> (f32, Vector2) {
        let adjacent = grid_pos.y - self.pos.y;

        let hypotenuse = adjacent / self.angle.to_radians().sin();

        let x = self.pos.x + hypotenuse * self.angle.to_radians().cos();

        let final_vec = Vector2::new(x, grid_pos.y);

        /*
        d.draw_circle(
            Self::entity_to_screen(&final_vec, &camera).x as i32,
            Self::entity_to_screen(&final_vec, &camera).y as i32,
            5.0,
            Color::RED,
        );
        */

        return (hypotenuse, final_vec);
    }

    /*


    */

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
        let ray_start_screen = Self::entity_to_screen(&self.pos, camera);
        let ray_end_screen = Self::entity_to_screen(&ray_end, camera);

        // Draw the ray
        d.draw_line_ex(ray_start_screen, ray_end_screen, 5.0, Color::RED);

        return ray_end;
    }

    fn entity_to_screen(entity_pos: &Vector2, camera: &Camera2D) -> Vector2 {
        Vector2::new(
            (entity_pos.x - camera.target.x) * camera.zoom + camera.offset.x,
            (entity_pos.y - camera.target.y) * camera.zoom + camera.offset.y,
        )
    }

    // This works now
    pub fn entity_to_world(entity: &Vector2, world: &World) -> Vector2 {
        Vector2::new(
            (entity.x / world.tile_size as f32).floor(),
            (entity.y / world.tile_size as f32).floor(),
        )
    }

    pub fn world_to_entity(world_pos: Vector2, world: &World) -> Vector2 {
        Vector2::new(
            world_pos.x * world.tile_size as f32,
            world_pos.y * world.tile_size as f32,
        )
    }

    fn draw_direction_line(&self, d: &mut RaylibDrawHandle, camera: &Camera2D) {
        // Transform the player's position to screen space
        let player_screen_pos = Self::entity_to_screen(&self.pos, camera);

        // Calculate the end point of the direction line in world space
        let direction_line_end_world = Vector2::new(
            self.pos.x + self.direction.x * 25.0, // No zoom here, as this is in world space
            self.pos.y + self.direction.y * 25.0,
        );

        // Transform the end point to screen space
        let direction_line_end_screen = Self::entity_to_screen(&direction_line_end_world, camera);

        // Draw direction line in screen space
        d.draw_line_ex(
            player_screen_pos,
            direction_line_end_screen,
            2.0 * camera.zoom,
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
                self.angle -= 5.0;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_S as c_int) {
                self.pos.x -= self.direction.x * self.velocity;
                self.pos.y -= self.direction.y * self.velocity;
            }
            if ffi::IsKeyDown(ffi::KeyboardKey::KEY_D as c_int) {
                self.angle += 5.0;
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
