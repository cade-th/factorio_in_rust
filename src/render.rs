use crate::Player;
use crate::World;
use raylib::prelude::*;

// TODO:
// 1. Add a state machine for zooming to have discrete zoom levels to avoid screen tearing

pub struct Renderer {
    pub camera: Camera2D,
}

impl Renderer {
    pub fn new(player: &Player) -> Self {
        let offset = unsafe {
            Vector2::new(
                raylib::ffi::GetScreenWidth() as f32 / 2.0,
                raylib::ffi::GetScreenHeight() as f32 / 2.0,
            )
        };

        Renderer {
            camera: Camera2D {
                target: Vector2::new(player.pos.x as f32, player.pos.y as f32),
                offset,
                rotation: 0.0,
                zoom: 1.0,
            },
        }
    }

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &mut World,
        player: &Player,
    ) {
        world.render(d, texture_atlas, &self.camera);
        player.render(d, &self.camera, world);
    }
}

pub fn entity_to_screen(entity_pos: Vector2, camera: &Camera2D) -> Vector2 {
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
