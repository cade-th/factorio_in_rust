use crate::world::Blocks;
use crate::world::World;
use raylib::prelude::*;

// Casts a number of rays spread out from the player
// Inputs: position, number of rays, angle to be cast (fov)
// Outputs: Array of distances and hitpoints for each ray

pub fn cast_fov(
    pos: Vector2,
    player_angle: f32,
    fov: f32,
    num_rays: i32,
    world: &World,
) -> (Vec<f32>, Vec<Vector2>) {
    let mut distances = Vec::new();
    let mut hit_positions = Vec::new();
    let start_angle = player_angle - (fov / 2.0); // Center the FOV around the player's angle
    let angle_step = fov / (num_rays - 1) as f32;

    for i in 0..num_rays {
        let angle = start_angle + i as f32 * angle_step;
        let (distance, hit_pos) = raycast_dda(pos, angle, world);
        distances.push(distance);
        hit_positions.push(hit_pos);
    }

    (distances, hit_positions)
}

// Takes the starting point and angle, and casts a ray into the world
// Inputs: Starting point, angle, world
// Outputs: distance of the ray, hit position
pub fn raycast_dda(start: Vector2, angle: f32, world: &World) -> (f32, Vector2) {
    let ray_angle = angle;
    let mut ray_pos = Vector2::new(0.0, 0.0);

    let mut y_offset = 0.0;
    let mut x_offset = 0.0;
    let mut dof: i32 = 0;
    let mut mx;
    let mut my;

    let mut dish = 100000.0;
    let mut hx = start.x;
    let mut hy = start.y;

    let mut atan = -1.0 / ray_angle.to_radians().tan();

    if ray_angle.to_radians().sin() < 0.0 {
        ray_pos.y = (((start.y as i32) >> 6) << 6) as f32 - 0.0001;
        ray_pos.x = start.x + (start.y - ray_pos.y) * atan;
        y_offset = -(world.tile_size as f32);
        x_offset = y_offset * atan;
    } else if ray_angle.to_radians().sin() > 0.0 {
        ray_pos.y = (((start.y as i32) >> 6) << 6) as f32 + world.tile_size as f32;
        ray_pos.x = start.x + (start.y - ray_pos.y) * atan;
        y_offset = world.tile_size as f32;
        x_offset = y_offset * atan;
    } else {
        ray_pos.x = start.x;
        ray_pos.y = start.y;
        dof = 8;
    }

    while dof < 8 {
        mx = (ray_pos.x as i32) >> 6;
        my = (ray_pos.y as i32) >> 6;

        if mx >= 0 && my >= 0 && mx < world.data.len() as i32 && my < world.data[0].len() as i32 {
            if world.data[mx as usize][my as usize] == Blocks::STONE {
                dof = 8;
                hx = ray_pos.x;
                hy = ray_pos.y;
                dish = dist(start.x, start.y, hx, hy);
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
    let mut vx = start.x;
    let mut vy = start.y;

    // Check Vertical Lines
    atan = -ray_angle.to_radians().tan();

    if ray_angle.to_radians().cos() < 0.0 {
        ray_pos.x = (((start.x as i32) >> 6) << 6) as f32 - 0.0001;
        ray_pos.y = start.y + (start.x - ray_pos.x) * atan;
        x_offset = -(world.tile_size as f32);
        y_offset = x_offset * atan;
    } else if ray_angle.to_radians().cos() > 0.0 {
        ray_pos.x = (((start.x as i32) >> 6) << 6) as f32 + world.tile_size as f32;
        ray_pos.y = start.y + (start.x - ray_pos.x) * atan;
        x_offset = world.tile_size as f32;
        y_offset = x_offset * atan;
    } else {
        ray_pos.x = start.x;
        ray_pos.y = start.y;
        dof = 8;
    }

    while dof < 8 {
        mx = (ray_pos.x as i32) >> 6;
        my = (ray_pos.y as i32) >> 6;

        if mx >= 0 && my >= 0 && mx < world.data.len() as i32 && my < world.data[0].len() as i32 {
            if world.data[mx as usize][my as usize] == Blocks::STONE {
                dof = 8;
                vx = ray_pos.x;
                vy = ray_pos.y;
                disv = dist(start.x, start.y, vx, vy);
            } else {
                ray_pos.x += x_offset;
                ray_pos.y -= y_offset;
                dof += 1;
            }
        } else {
            dof = 8;
        }
    }

    let final_dist;

    if disv < dish {
        ray_pos.x = vx;
        ray_pos.y = vy;
        final_dist = disv;
    } else {
        ray_pos.x = hx;
        ray_pos.y = hy;
        final_dist = dish;
    }

    return (final_dist, ray_pos);
}

pub fn dist(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt()
}

pub fn ray_cast_brute_force(start: Vector2, angle: f32, world: &mut World) -> Vector2 {
    // Start the ray in world space at the player's position
    let mut ray_end = start;
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
        ray_end.x += angle.to_radians().cos() * step_size;
        ray_end.y += angle.to_radians().sin() * step_size;
    }

    return ray_end;
}
