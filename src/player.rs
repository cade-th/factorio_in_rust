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
            Self::entity_to_screen(self.pos, camera).x as i32,
            Self::entity_to_screen(self.pos, camera).y as i32,
            player_radius,
            Color::RED,
        );

        Self::raycast_dda_2(self, d, camera, world);
        // self.ray_cast_brute_force(d, world, camera);
    }

    pub fn raycast_dda(&self, d: &mut RaylibDrawHandle, camera: &Camera2D, world: &World) {
        let ray_start = self.pos;
        let ray_angle = self.angle;

        let mut grid_pos = Self::entity_to_world(self.pos, world);

        if ray_angle.to_radians().cos() < 0.0 {
            grid_pos.x += 1.0;
        }

        grid_pos = Self::world_to_entity(grid_pos, world);

        // =================================================================================

        let intersection_1 =
            Self::find_vertical(&mut grid_pos, world, camera, d, ray_angle, ray_start);
        let intersection_2 =
            Self::find_vertical(&mut grid_pos, world, camera, d, ray_angle, intersection_1);
        let _intersection_3 =
            Self::find_vertical(&mut grid_pos, world, camera, d, ray_angle, intersection_2);

        // =================================================================================

        /*

        grid_pos = Self::entity_to_world(self.pos, world);

        if ray_angle.to_radians().sin() < 0.0 {
            grid_pos.y += 1.0;
        }

        grid_pos = Self::world_to_entity(grid_pos, world);

        //let _intersection_horizontal =
        let intersection_4 =
            Self::find_horizontal(&mut grid_pos, world, camera, d, ray_angle, ray_start);
        let intersection_5 =
            Self::find_horizontal(&mut grid_pos, world, camera, d, ray_angle, intersection_4);
        let intersectoin_6 =
            Self::find_horizontal(&mut grid_pos, world, camera, d, ray_angle, intersection_5);

        */
    }

    pub fn dist(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
        ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt()
    }

    pub fn raycast_dda_2(&self, d: &mut RaylibDrawHandle, camera: &Camera2D, world: &World) {
        let ray_angle = self.angle;
        let mut ray_pos = Vector2::new(0.0, 0.0);

        let mut y_offset = 0.0;
        let mut x_offset = 0.0;
        let mut dof: i32 = 0;
        let mut mx;
        let mut my;

        let mut dish = 100000.0;
        let mut hx = self.pos.x;
        let mut hy = self.pos.y;

        // Check Horizontal Lines
        let mut atan = -1.0 / ray_angle.to_radians().tan();

        if ray_angle.to_radians().sin() < 0.0 {
            ray_pos.y = (((self.pos.y as i32) >> 6) << 6) as f32 - 0.0001;
            ray_pos.x = self.pos.x + (self.pos.y - ray_pos.y) * atan;
            y_offset = -(world.tile_size as f32);
            x_offset = y_offset * atan;
        } else if ray_angle.to_radians().sin() > 0.0 {
            ray_pos.y = (((self.pos.y as i32) >> 6) << 6) as f32 + world.tile_size as f32;
            ray_pos.x = self.pos.x + (self.pos.y - ray_pos.y) * atan;
            y_offset = world.tile_size as f32;
            x_offset = y_offset * atan;
        } else {
            ray_pos.x = self.pos.x;
            ray_pos.y = self.pos.y;
            dof = 8;
        }

        while dof < 8 {
            mx = (ray_pos.x as i32) >> 6;
            my = (ray_pos.y as i32) >> 6;

            if mx >= 0 && my >= 0 && mx < world.data.len() as i32 && my < world.data[0].len() as i32
            {
                if world.data[mx as usize][my as usize] == Blocks::STONE {
                    dof = 8;
                    hx = ray_pos.x;
                    hy = ray_pos.y;
                    dish = Self::dist(self.pos.x, self.pos.y, hx, hy);
                } else {
                    ray_pos.x -= x_offset;
                    ray_pos.y += y_offset;
                    dof += 1;
                }
            } else {
                dof = 8;
            }
        }

        dof = 0;
        let mut disv = 100000.0;
        let mut vx = self.pos.x;
        let mut vy = self.pos.y;

        // Check Vertical Lines
        atan = -ray_angle.to_radians().tan();

        if ray_angle.to_radians().cos() < 0.0 {
            ray_pos.x = (((self.pos.x as i32) >> 6) << 6) as f32 - 0.0001;
            ray_pos.y = self.pos.y + (self.pos.x - ray_pos.x) * atan;
            x_offset = -(world.tile_size as f32);
            y_offset = x_offset * atan;
        } else if ray_angle.to_radians().cos() > 0.0 {
            ray_pos.x = (((self.pos.x as i32) >> 6) << 6) as f32 + world.tile_size as f32;
            ray_pos.y = self.pos.y + (self.pos.x - ray_pos.x) * atan;
            x_offset = world.tile_size as f32;
            y_offset = x_offset * atan;
        } else {
            ray_pos.x = self.pos.x;
            ray_pos.y = self.pos.y;
            dof = 8;
        }

        while dof < 8 {
            mx = (ray_pos.x as i32) >> 6;
            my = (ray_pos.y as i32) >> 6;

            if mx >= 0 && my >= 0 && mx < world.data.len() as i32 && my < world.data[0].len() as i32
            {
                if world.data[mx as usize][my as usize] == Blocks::STONE {
                    dof = 8;
                    vx = ray_pos.x;
                    vy = ray_pos.y;
                    disv = Self::dist(self.pos.x, self.pos.y, vx, vy);
                } else {
                    ray_pos.x += x_offset;
                    ray_pos.y -= y_offset;
                    dof += 1;
                }
            } else {
                dof = 8;
            }
        }

        // Compare distances and set the final ray position
        if disv < dish {
            ray_pos.x = vx;
            ray_pos.y = vy;
        } else {
            ray_pos.x = hx;
            ray_pos.y = hy;
        }

        // Draw the final ray position
        d.draw_circle(
            Self::entity_to_screen(ray_pos, &camera).x as i32,
            Self::entity_to_screen(ray_pos, &camera).y as i32,
            10.0,
            Color::BLUE,
        );
    }

    pub fn find_vertical(
        grid_pos: &mut Vector2,
        world: &World,
        camera: &Camera2D,
        d: &mut RaylibDrawHandle,
        ray_angle: f32,
        ray_start: Vector2,
    ) -> Vector2 {
        *grid_pos = Self::entity_to_world(*grid_pos, world);

        if ray_angle.to_radians().cos() > 0.0 {
            grid_pos.x += 1.0;
        } else {
            grid_pos.x -= 1.0;
        }
        *grid_pos = Self::world_to_entity(*grid_pos, world);

        let opp = (grid_pos.x - ray_start.x) * ray_angle.to_radians().tan();

        let ray_length = opp / ray_angle.to_radians().cos();

        let y = ray_start.y + opp;

        let final_vec = Vector2::new(grid_pos.x, y);

        d.draw_circle(
            Self::entity_to_screen(final_vec, &camera).x as i32,
            Self::entity_to_screen(final_vec, &camera).y as i32,
            5.0,
            Color::BLUE,
        );
        final_vec
    }

    pub fn find_horizontal(
        grid_pos: &mut Vector2,
        world: &World,
        camera: &Camera2D,
        d: &mut RaylibDrawHandle,
        ray_angle: f32,
        ray_start: Vector2,
    ) -> Vector2 {
        *grid_pos = Self::entity_to_world(*grid_pos, world);

        if ray_angle.to_radians().sin() > 0.0 {
            grid_pos.y += 1.0;
        } else {
            grid_pos.y -= 1.0;
        }
        *grid_pos = Self::world_to_entity(*grid_pos, world);

        let adj = (grid_pos.y - ray_start.y) / ray_angle.to_radians().tan();

        let ray_length = adj / ray_angle.to_radians().sin();

        let x = ray_start.x + adj;

        let final_vec = Vector2::new(x, grid_pos.y);

        d.draw_circle(
            Self::entity_to_screen(final_vec, &camera).x as i32,
            Self::entity_to_screen(final_vec, &camera).y as i32,
            5.0,
            Color::RED,
        );

        return final_vec;
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
                Self::entity_to_screen(input_x.1, camera).x as i32,
                Self::entity_to_screen(input_x.1, camera).y as i32,
                8.0,
                Color::GREEN,
            );
            return (input_x.1, true);
        } else {
            d.draw_circle(
                Self::entity_to_screen(input_y.1, camera).x as i32,
                Self::entity_to_screen(input_y.1, camera).y as i32,
                8.0,
                Color::GREEN,
            );
            return (input_y.1, false);
        }
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
    pub fn entity_to_world(entity: Vector2, world: &World) -> Vector2 {
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
        let player_screen_pos = Self::entity_to_screen(self.pos, camera);

        // Calculate the end point of the direction line in world space
        let direction_line_end_world = Vector2::new(
            self.pos.x + self.direction.x * 25.0, // No zoom here, as this is in world space
            self.pos.y + self.direction.y * 25.0,
        );

        // Transform the end point to screen space
        let direction_line_end_screen = Self::entity_to_screen(direction_line_end_world, camera);

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
